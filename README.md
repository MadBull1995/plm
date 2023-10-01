# PLM - Protobuf Library Manager

[![CI](https://github.com/MadBull1995/plm/actions/workflows/ci.yml/badge.svg)](https://github.com/MadBull1995/plm/actions/workflows/ci.yml)
[![Docker](https://img.shields.io/docker/pulls/sylkbuild/plm.svg)](https://hub.docker.com/r/sylkbuild/plm/)

---

## Introduction

PLM is an open-source Protobuf Library Manager that allows developers to easily publish, share, and manage protocol buffers libraries. Whether you are a seasoned developer or a beginner, PLM offers a streamlined process for protobuf management, saving you time and effort.

## Features
- Setup Registry instance.
- Publish your own libraries with a single command.
- Setup a locl workspace.
- Install and manage dependencies with ease.
- Secure user authentication.

## Table of Contents

1. [Getting Started](#getting-started)
    - [Installation](#installation)
    - [Usage](#usage)
2. [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Building from Source](#building-from-source)
3. [API Documentation](#api-documentation)
4. [How to Contribute](#how-to-contribute)
5. [License](#license)
6. [Contact](#contact)
7. [Acknowledgements](#acknowledgements)

## Getting Started

### Installation

Install the `plm-cli` via cargo:
```bash
cargo install plm-cli
```

### Usage

Setup the `plm-registry` instance:

#### Locally
```bash
# Clone the repo
git clone https://github.com/[YourUsername]/plm.git

# Navigate into the directory
cd plm

# Run locally the instance
cargo run --path plm-registry
```

#### Dockerized
```bash
# Clone the repo
git clone https://github.com/[YourUsername]/plm.git

# Navigate into the directory
cd plm

# Build and run the containers
docker-compose up --build -d
```

Then you can login to the registry:

```bash
plm login username password
```

To setup a workspace:
```bash
plm init
```

To publish a library:
```bash
plm publish
```

To install a library:

```bash
plm install my-library
```

## How to Contribute

We welcome contributions from the community. To get started, please fork the repository and submit a pull request.

## License

This project is licensed under the APACHE-2.0 License - see the [LICENSE](LICENSE) file for details.