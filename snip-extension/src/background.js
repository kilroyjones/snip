let toggleAdd = false;
let currentSelection = "";

chrome.runtime.onInstalled.addListener(() => {
  console.log("INSTALLED");
  chrome.storage.local.set({ snipSearchBar: false });
});

function sendQuoteToServer(sourceURL, title, quote) {
  (async () => {
    const response = await fetch("http://127.0.0.1:8080/add-quote", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        quote: currentSelection,
        source: sourceURL,
        description: title,
        tags: [""],
      }),
    });
    // ADD ERROR HANDLING
  })();
}

function sendBookmarkToServer(sourceUrl, linkUrl) {
  console.log("SOURCE:", sourceUrl);
  console.log("LINK", linkUrl);
  (async () => {
    const response = await fetch("http://127.0.0.1:8080/add-bookmark", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        link: linkUrl,
        description: "",
        tags: [""],
      }),
    })
      .then(res => {
        console.log("success: ", res);
        return res;
      })
      .catch(err => {
        console.log("error");
      });

    // ADD ERROR HANDLING
  })();
}

function sendImageToServer(title, imageUrl, sourceUrl) {
  (async () => {
    const response = await fetch("http://127.0.0.1:8080/add-image", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        url: imageUrl,
        source: sourceUrl,
        description: title,
        tags: [""],
      }),
    })
      .then(res => {
        console.log("success");
        return res;
      })
      .catch(err => {
        console.log("error");
      });
    // ADD ERROR HANDLING
  })();
}

/* GRAB QUOTE */
async function snipHandlerQuote(snip, tab) {
  console.log("Quote");
  await sendQuoteToServer(tab.url, tab.title, snip.selectionText);
}

async function snipHandlerImage(snip, tab) {
  console.log("Image");
  await sendImageToServer(tab.title, snip.srcUrl, snip.pageUrl);
}

async function snipHandlerBookmark(snip, tab) {
  console.log("Bookmark");
  await sendBookmarkToServer(tab.url, snip.linkUrl);
}

chrome.runtime.onInstalled.addListener(function () {
  chrome.contextMenus.removeAll();

  chrome.contextMenus.create({
    type: "normal",
    contexts: ["selection"],
    id: "snipHandlerQuote",
    title: "Snip quote",
  });

  chrome.contextMenus.create({
    type: "normal",
    contexts: ["image"],
    id: "snipHandlerImage",
    title: "Snip image",
  });

  chrome.contextMenus.create({
    type: "normal",
    contexts: ["link"],
    id: "snipHandlerBookmark",
    title: "Snip link",
  });
});

chrome.contextMenus.onClicked.addListener(async function (snip) {
  await console.log("Here", snip);
  let tab = await getTab();
  console.log("Tag");
  // chrome.scripting
  //   .executeScript({
  //     target: { tabId: tab.id },
  //     func: await findElement,
  //     args: [info.mediaType, info.srcUrl],
  //   })
  //   .then(res => {
  //     console.log("sadsfkjadf", res.result);
  //   });
  if ("selectionText" in snip) {
    console.log("snipping quote");
    await snipHandlerQuote(snip, tab);
  } else if ("mediaType" in snip) {
    if (snip.mediaType === "image") {
      console.log("snipping image", snip);
      await snipHandlerImage(snip, tab);
    }
  } else {
    console.log("snipping link");
    await snipHandlerBookmark(snip, tab);
  }
});
// this function's code will be executed as a content script in the web page
function findElement(mediaType, srcUrl) {
  const tagName = mediaType === "image" ? "img" : mediaType;
  for (const el of document.querySelectorAll(tagName)) {
    if (el.src === srcUrl) {
      console.log(el);
      return el.alt;
    }
  }
  return null;
}

async function getTab() {
  let queryOptions = { active: true, currentWindow: true };
  let tab = await chrome.tabs.query(queryOptions);
  return tab[0];
}

async function processKeypress() {}

chrome.commands.onCommand.addListener(async command => {
  console.log(`Command "${command}" triggered`);
  getTab().then(async tab => {
    console.log("Here");
    chrome.tabs.sendMessage(tab.id, "toggle-search", function (response) {
      console.log(response);
    });
  });
});

chrome.runtime.onMessage.addListener((message, sender, sendMessage) => {
  console.log(message);
  if (message.status === "updateSelection") {
    currentSelection = message.selection;
    console.log("CS", currentSelection);
  } else {
    console.log("msg", message, sender, sendMessage);
  }
});
