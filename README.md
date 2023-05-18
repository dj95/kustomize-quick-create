<h1 align="center">kqc 🚀☸️</h1>

<p align="center">
  A quick create wizard to create and modify opinionated kustomize deployments.
  <br><br>
  This tool should help to quickly create and build up kustomize deployments. It features a wizard, which
  will let you create the boilerplate configuration and underlying file structure. With a templateing
  engine the most important values are already set into the resources, such that only minor further tweaks
  should be required for getting the deployment up and running.
  <br><br>
  <a href="https://github.com/dj95/kubernetes-quick-create/releases">
    <img alt="latest version" src="https://img.shields.io/github/tag/dj95/kubernetes-quick-create.svg" />
  </a>
</p>


### 📦 Requirements

- rust


### 🚀 Getting started

Clone the repository and run `cargo install --path .`. Then you should be able to call the wizard with `kustomize-quick-create`.
It will detect the kustomize deployment if it exists in the `kubernetes/` subdirectory of the present working dir.


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
