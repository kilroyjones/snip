# Snip server

## Table of contents

- [Introduction](https://github.com/kilroyjones/snip-server#Introduction)
- [Motivation](https://github.com/kilroyjones/snip-server#Motivation)
- [Installation](https://github.com/kilroyjones/snip-server#Installation)
- [Overview of the software](https://github.com/kilroyjones/snip-server#Overview-of-the-software)
- [Development roadmap](https://github.com/kilroyjones/snip-server#Development-roadmap)
- [Licensing](https://github.com/kilroyjones/snip-server#Licensing)

## Introduction

Snip is a self-hosted tool to save bookmarks, quotes, images, files and more from the web. There are three main components:

- **The server**, which is where your data is stored (this repo).
- [**The frontend**](https://github.com/kilroyjones/snip-frontend), where you'll interact with everything on the server.
- [**The browser extension**](https://github.com/kilroyjones/snip-extension), that allows you to save things as you browse.

The basic flow for use would to be add items via the plugin, which are then sent to the server. There, the server will process them as need be. That might mean creating an image thumbnail or scanning a quote (selected text) for code which needs to be formatted. The front end is then used to view sort and search through your collection.

## Motivation

I began working on this out of the desire to have a better way to collect, store and search through material I find interesting. What I wanted was something that could be self-hosted, whether running on one's own machine or online.

## Installation

To run the server first clone the repo, move into that folder and then run:

```bash
cargo run
```

## Overview of the software

At the present the server is 100% written in rust, though in looking at some of the features I'd like to add, there may be a need to include other languages. Finding a library which can determine the programming language of a code snippet has been difficult within Rust. The **guesslang** model can be easily used from Python, and I'm contemplating using that module, as I've had difficulty porting Tensorflow model into Rust.

## Development roadmap

As already noted, this is a work in progress. What follows is a rough outline of milestones and server features.

### v0.1

- [x] - Quotes saved
- [x] - Links saved
- [x] - Images saved
- [x] - Thumbnails processed
- [ ] - Paging
- [ ] - Tags
- [ ] - Filter by date
- [ ] - Filter by tags
- [ ] - Find and format code snippets in text selection
- [ ] - Basic search
- [ ] - Logging
- [ ] - Single-user authentication
- [ ] - Package as container

### v0.2

- [ ] - Advanced search features
- [ ] - Automated backups
- [ ] - Videos saved
- [ ] - Files saved
- [ ] - Screenshots vs. images
- [ ] - Automated backup mechanism
- [ ] - Option for encrypting data

## Licensing

MIT Licensed
