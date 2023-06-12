<h1 align="center">kqc 🚀☸️</h1>

<p align="center">
  A quick create wizard to create and modify opinionated kustomize deployments.
  <br><br>
  <a href="https://github.com/dj95/kustomize-quick-create/releases">
    <img alt="latest version" src="https://img.shields.io/github/v/tag/dj95/kustomize-quick-create.svg?sort=semver" />
  </a>
  <br><br>
  This tool should help to quickly create and build up kustomize deployments. It features a wizard, which
  will let you create the boilerplate configuration and underlying file structure. With a templateing
  engine the most important values are already set into the resources, such that only minor further tweaks
  should be required for getting the deployment up and running.
</p>


![Demo GIF of kqc in action](./example/demo.gif)


### 📦 Requirements

- rust

*or*

- nix
- direnv


### 🚀 Getting started

Clone the repository and make sure the dependencies are installed. You either need rust or nix installed.
With nix use either `nix-shell` or `direnv allow` up to your preferences.
After dependencies are available run `cargo install --path .` to build and install the tool.

Then you should be able to call the wizard with `kustomize-quick-create`.
It will detect the kustomize deployment if it exists in the `kubernetes/` subdirectory of the present working dir.


### 🧪 Customizing the templates

Templates are stored in the *templates/* directory of this repository and statically compiled into the binary.
Feel free to tinker around with the templates and customize them to your needs.
Please keep in mind that you need to update the fields of the relating struct in *src/resources* if you change them.
Simply search for the template name in the code base and you should be able to find them.


### ❄️ Installation with nix


Add the following code to your overlays. Then kustomize-quick-create can be installed from `pkgs`.

```nix
final: prev: {
  kustomize-quick-create = prev.pkgs.rustPlatform.buildRustPackage rec {
    version = "0.1.0";
    pname = "kustomize-quick-create";

    src = prev.fetchFromGitHub {
      owner = "dj95";
      repo = pname;
      rev = "f2c811652ab824616fb48cdbc8c78d1e8b83d8c0";
      sha256 = "sha256-q6Cu0o5P3Tic3a3H9OOash6HbdJxciNhkKJQxd0WEaU=";
    };

    cargoSha256 = "sha256-Ay96cF3H3l7/yzU5ejOrK96T9WoEaIujiEM285qxMFU=";
  };
}
```

If you just want to try it with nix, run the following command.

```bash
nix run 'github:dj95/kustomize-quick-create'
```


## 🤝 Contributing

If you are missing features or find some annoying bugs please feel free to submit an issue or a bugfix within a pull request :)


## 📝 License

© 2023 Daniel Jankowski


This project is licensed under the MIT license.


Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:


The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.


THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
