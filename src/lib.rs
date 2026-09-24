#![deny(clippy::all)]

use std::{
  fs,
  path::{Component, Path, PathBuf},
  sync::Arc,
};

use farmfe_core::{
  config::Config,
  context::CompilationContext,
  error::{CompilationError, Result},
  plugin::{Plugin, PluginHookContext, PluginLoadHookParam, PluginLoadHookResult},
};
use farmfe_macro_plugin::farm_plugin;

#[farm_plugin]
pub struct FarmPluginCaseSensitive {}

impl FarmPluginCaseSensitive {
  fn new(_config: &Config, _options: String) -> Self {
    Self {}
  }
}

// 逐级检查路径每一段的大小写是否与文件系统中的实际名称一致。
fn find_case_mismatch(path: &Path) -> Option<(PathBuf, PathBuf)> {
  let mut current = PathBuf::new();

  for component in path.components() {
    match component {
      Component::Prefix(_) | Component::RootDir | Component::CurDir => {
        current.push(component.as_os_str());
      }
      Component::ParentDir => {
        current.pop();
      }
      Component::Normal(expected) => {
        let parent = current.clone();
        // 使用不区分大小写的匹配找到磁盘上的真实路径段。
        let actual = fs::read_dir(&parent)
          .ok()?
          .filter_map(|entry| entry.ok())
          .find_map(|entry| {
            let name = entry.file_name();
            name
              .to_string_lossy()
              .eq_ignore_ascii_case(&expected.to_string_lossy())
              .then(|| parent.join(name))
          })?;

        current.push(expected);
        if actual.file_name()? != expected {
          return Some((current, actual));
        }
      }
    }
  }

  None
}

impl Plugin for FarmPluginCaseSensitive {
  fn name(&self) -> &str {
    "FarmPluginCaseSensitive"
  }

  fn load(
    &self,
    param: &PluginLoadHookParam,
    _context: &Arc<CompilationContext>,
    _hook_context: &PluginHookContext,
  ) -> Result<Option<PluginLoadHookResult>> {
    // Farm 已经完成模块解析，这里检查最终加载文件的真实路径。
    let resolved_path = Path::new(param.resolved_path);

    if let Some((requested, actual)) = find_case_mismatch(resolved_path) {
      return Err(CompilationError::GenericError(format!(
        "文件路径大小写不匹配: 请求 '{}', 实际 '{}'. 请修正 import/require 路径的大小写。",
        requested.display(),
        actual.display()
      )));
    }

    Ok(None)
  }
}
