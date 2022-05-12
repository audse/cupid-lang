use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportNode {
	PackageList(Vec<Package>),
	Package(Package),
	NameSpace(NameSpace),
	Items(Vec<ImportItem>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
	pub name_space: Option<NameSpace>,
	pub items: Vec<ImportItem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameSpace {
	pub name: Cow<'static, str>,
	pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
	pub identifier: Cow<'static, str>,
	pub tokens: Vec<Token>
}

impl ImportNode {
	pub fn use_packages(&self) -> Vec<PackageContents> {
		let mut package_list: Vec<PackageContents> = vec![];
		if let Self::PackageList(packages) = self {
			for package in packages {
				let contents = get_contents(&package);
				package_list.push(get_contents(&package));
			}
		} else {
			panic!("expected package list")
		}
		package_list
	}
}

fn get_contents(package: &Package) -> PackageContents {
	let folder = if let Some(name_space) = &package.name_space {
		name_space.name.to_string() + "/"
	} else {
		String::new()
	};
	let file_name = if package.items.len() == 1 {
		&*package.items[0].identifier
	} else {
		panic!("no file name")
	};
	let path = "./../".to_string() + folder.as_str() + &*file_name + ".cupid";
	PackageContents {
		contents: read(&path),
		path,
	}
}

fn read(path: &str) -> String {
	std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to find file at path {path}"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageContents {
	pub path: String,
	pub contents: String,
}