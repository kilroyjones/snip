# Snip frontend

## Table of contents

- [Introduction](https://github.com/kilroyjones/snip-server#Introduction)
- [Installation](https://github.com/kilroyjones/snip-server#Installation)
- [Overview of the software](https://github.com/kilroyjones/snip-server#Overview-of-the-software)
- [Development roadmap](https://github.com/kilroyjones/snip-server#Development-roadmap)
- [Licensing](https://github.com/kilroyjones/snip-server#Licensing)

## Introduction

This is the frontend of the self-hosted [Snip server](https://github.com/kilroy-jones/snip-server) which is used to save bookmarks, quotes, images, files and more from the web. There are three main components:

- [**The server**](https://github.com/kilroyjones/snip-server),\*The server\*\*, which is where your data is stored.
- **The frontend**, where you'll interact with everything on the server (this repo).
- [**The browser extension**](https://github.com/kilroyjones/snip-extension), that allows you to save things as you browse.

To learn more about the motivation behind this project, see the [**Snip server**](https://github.com/kilroyjones/snip-server) repo.

## Running

To run the frontend it's necessary to first install the server, which you an find [here](https://github.com/kilroyjones/snip-server). Once that's running you should clone this repo, and from it's root folder, running the following:

```bash
npm install
npm run dev
```

This will start the development server. Alternatvely, you can build the product and depoly it on your own, but as of yet no production deployed has been implemented.

## Overview of the software

This project is written in Svelte using Typescript, and uses the latest pre-1.0 version. The choice of Svelte was primarily due to my familiarity with it, and just that I enjoy using it. The code is still rough, and changes will be coming somewhat regularly.

## Development roadmap

As already noted, this is a work in progress. What follows is a rough outline for myself.

### v0.1

- [x] - Quotes page
- [ ] - Links page
- [x] - Images page
- [x] - Side toolbar
- [ ] - Search and filtering
- [ ] - Tags for all entries
- [ ] - Light and dark modes
- [ ] - Account management
- [ ] - Code syntax formatting within quotes
- [ ] - Toggle results features

### v0.2

- [ ] - Allow connecting of users (follow)

## Licensing

MIT Licensed
