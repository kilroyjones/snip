(function () {
  'use strict';

  // chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
  //   console.log(sender.tab ? "from a content script:" + sender.tab.url : "from the extension");
  //   if (request.greeting === "hello") sendResponse({ farewell: "goodbye" });
  //   return true;
  // });

  document.addEventListener(
    "contextmenu",
    function (event) {
      console.log("Fuck", event);
    },
    true
  );

})();
//# sourceMappingURL=content.js.map
