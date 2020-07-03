# CSML Conversational Engine

![CSML logo](./images/csml-horizontal-whitebg-v3.png)

## Introduction

The CSML (Conversational Standard Meta Language) is a Domain-Specific Language developed for creating conversational experiences easily.

The purpose of this language is to simplify the creation and maintenance of rich conversational interactions between humans and machines. With a very expressive and text-only syntax, CSML flows are easy to understand, making it easy to deploy and maintain conversational agents. The CSML handles short and long-term memory slots, metadata injection, and connecting to any third party API or injecting arbitrary code in any programming language thanks to its powerful runtime APIs.

By using the CSML language, any developer can integrate arbitrarily complex conversational agents on any channel (Facebook Messenger, Slack, Facebook Workplace, Microsoft Teams, custom webapp, ...) and make any bot available to any end user. The CSML platform comes with a large number of channel integrations that work out of the box, but developers are free to add new custom integrations by using the CSML interfaces.

## Usage

The CSML Engine and Language are built in Rust. The full documentation of the project is available on https://docs.csml.dev.

The conversational engine is available for use in several types of projects, depending on your environment of choice.

### With CSML Studio

The simplest way to get started with CSML is to use CSML Studio, a free online environment with everything already setup to start creating bots right away, directly in your browser.

To get started with CSML Studio: https://studio.csml.dev

CSML Studio gives you a free playground to experiment with the language as well as options to deploy your chatbots at scale in one-click.

### With Docker

We also provide a docker image for easy self-hosted usage.

```
docker pull clevy/csml-engine
```

To get started with CSML Engine on Docker: https://github.com/CSML-by-Clevy/csml-engine-docker

### With Rust

(Pending documentation)

### With nodejs

This repository provides nodejs bindings of this rust library. To use this library in a nodejs project, you will need to build it from source. There are a few requirements:

- Rust v1.44
- Nodejs LTS or above
- Neon CLI v0.4.0 (make sure that all [required dependencies](https://neon-bindings.com/docs/getting-started/#install-node-build-tools/) are installed)
- libssl-dev (or equivalent for your architecture: openssl-dev, libssl-devel...)

To compile CSML Engine into a [native node module](https://nodejs.org/api/addons.html), run:

```shell
git clone https://github.com/CSML-by-Clevy/csml-engine csml
neon build -p csml/bindings/node --release
```

> If you are not familiar with Rust build times, please know that the `neon build` step can take up to 10 minutes. Be patient!

This method will output this native file: `csml/bindings/node/native/index.node` that you can simply `require()` (or `import`) in your project. For more details about how to use this module in your own projects, you can have a look at [our implementation for Docker version](https://github.com/CSML-by-Clevy/csml-engine-docker/blob/master/app/server.js).

Please note that if you plan to deploy your project on a different architecture, you will need to recompile the project on that architecture. We recommend using git submodules if you need to integrate CSML Engine in your own nodejs projects.

## Additional Information

### Play with the language

* [Studio] - Create and deploy your chatbot in a matter of minutes.

[Studio]: https://studio.csml.dev

### Getting Help

* [Slack] - Direct questions about using the language.
* [CSML Documentation](https://docs.csml.dev) - Getting started.

[Slack]: https://csml-by-clevy.slack.com/join/shared_invite/enQtODAxMzY2MDQ4Mjk0LWZjOTZlODI0YTMxZTg4ZGIwZDEzYTRlYmU1NmZjYWM2MjAwZTU5MmU2NDdhNmU2N2Q5ZTU2ZTcxZDYzNTBhNTc

### Information

* [Roadmap](https://trello.com/b/tZ1MoALL/csml-open-roadmap) - Upcoming new features.
* [Release notes](https://headwayapp.co/csml-release-notes) - Stay up to date.
