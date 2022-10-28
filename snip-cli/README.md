# Snip description

Snip is a tool which allows you to quickly save segments of code and other bits of information that you find important. You can add tags and a description and then search, sort, and save then as you want.

### What can you snip?

You can snip any text or text-based document directly from the clipboard or by giving Snip a file or files. If you look at the roadmap, the plan is to add image and formatted text functionality going forward.

### How to install

### How to use

### How it works

Whenever you snip something and save locally, it's stored in a SQLite database located in _/etc/snip/_. The database structure can be found by looking at the _up.sql_ file located in **\*\***\_\_\_**\*\*** of the repo.

The database is set up for full text search using FTS5 with basic searching capabilities. See the above **How to use** section for more detail on how to search.

### Development roadmap

Currently, the program is only available locally. The plan is to create something which allows one to snip from the browser as well as the command line, with syncing data to your online store and allowing you to share with others. Going forward the project is broken into three development stages: local, web-based, and community interaction.

#### Local

[ ] - Save from clipboard
[ ] - Save single or multiples files
[ ] - Allow for tags
[ ] - Allow for descriptions
[ ] - Boolean (and/or/not) CLI search
[ ] - Interactive mode for adding
[ ] - Search? (see comment below)
[ ] - Export to folders based on tags or given file types

Still figuring out best way to do search. Using a listing with preview and then select or creating a simple TUI with file listing and preview is another option.

#### Web

The goal would be to allow people to create and access snippets from both the CLI and the browser. Being able to quickly highlight and save off snippets within the context of the current page would be helpful.

[ ] - Server as local service
[ ] - Option for Postgresql backend
[ ] - Browser extension allows saving of snippets and whole pages
[ ] - Storing within context of page contents (need to be able to read HTML and PDFs)
[ ] - Web-based snippet search
[ ] - Web-based organizer

To allow for non-local hosting, I'll need to figure out how to do the auth, some options are using an existing SSH connection or providing a renewable token.

#### Community

The long-term goal would be to create a community of curated snippets that can be shared, saved, forked, and commented on.

[ ] - Account system
[ ] - Allow for a personal store
[ ] - Syncing personal store with online (could I use git for this?)

### Questions

1. Why not just use gists?

- While gists are searchable, they cannot be tagged or sorted. They are also not stored locally nor owned by you.
