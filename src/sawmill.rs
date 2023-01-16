use std::{fs, path::PathBuf, io::{Write, BufReader, BufRead}, ops::Deref, str::Lines};

use anyhow::Result;


pub struct Changelog {
  file: fs::File,
  old: Option<String>,
  project_path: PathBuf
}

impl Changelog {
  pub fn new(mut file: fs::File, old: Option<String>, project_path: PathBuf) -> Self {
    match &old {
      Some(old_str) => {
        let title_line = match old_str.lines().next() {
          Some(hl) =>  hl,
          None => { "" }
        };
        writeln!(file, "{}", title_line);
      }

      None => {}
    }

    Self {
      file,
      old,
      project_path
    }
  }

  pub fn update(&mut self, change_groups: &Vec<&Vec<String>>) {
    for group in change_groups {
      self.write_change_group("0.0.0", group);
    }
  }

  pub fn close(mut self) {
    if let Some(old) = self.old {
      for line in old.split_inclusive('\n').skip(1) {
        self.file.write(line.as_bytes());
      }

      fs::rename(&self.project_path.join("CHANGELOG.tmp.md"), &self.project_path.join("CHANGELOG.md"));
    }
  }

  fn write_change_group(&mut self, group_name: &str, entries: &Vec<String>) -> Result<()> {
    writeln!(self.file, "### {}", group_name)?;
    writeln!(self.file)?;
    for entry in entries.iter() {
      writeln!(self.file, "{}", entry)?;
    }

    writeln!(self.file)?;

    Ok(())
  }

}

pub struct Sawmill {
  project_path: PathBuf,
  changelog_path: PathBuf
}

impl Sawmill {
  pub fn new(project_path: &PathBuf) -> Self {
    Self {
      project_path: PathBuf::from(project_path),
      changelog_path: project_path.join("CHANGELOG.md")
    }
  }

  pub fn open_changelog(self) -> Result<Changelog> {
    if self.is_changelog_present()? {
      let file = fs::File::open(&self.changelog_path)?;
      let changelog_str = fs::read_to_string(&self.changelog_path)?;
      let mut tmp_changelog_file = fs::File::create(&self.project_path.join("CHANGELOG.tmp.MD"))?;

      Ok(Changelog::new(tmp_changelog_file, Some(changelog_str), self.project_path))
    } else {
      let mut new_changelog_file = self.generate_new_changelog("project_name")?;
      Ok(Changelog::new(new_changelog_file, None, self.project_path))
    }
  }

  fn is_changelog_present(&self) -> Result<bool> {
    Ok(self.changelog_path.try_exists()?)
  }

  fn generate_new_changelog(&self, project_name: &str) -> Result<fs::File> {
    let mut changelog_file = fs::File::create(&self.changelog_path)?;
    writeln!(changelog_file, "# {}", project_name)?;
    writeln!(changelog_file)?;

    Ok(changelog_file)
  }
}






