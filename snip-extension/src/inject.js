if (typeof init === "undefined") {
  // Add bubble to the top of the page.
  const hostElement = document.createElement("div");
  hostElement.className = "hostElement";
  hostElement.style.visibility = "hidden";
  document.body.appendChild(hostElement);
  var host = document.querySelector(".hostElement");
  var root = host.attachShadow({ mode: "open" });
  var div = document.createElement("div");
  div.className = "div root-class";
  div.innerHTML = `
    <style>
    body {
      width: 100%;
      height: 100vh;
    }

    #add-snippet {
      top: 50%; /* IMPORTANT */
      left: 50%; /* IMPORTANT */
      display: block;
      position: absolute;
      width: 600px;
      max-width: 600px;
      height: 80px;
      background-color: #e6efe9;
      margin-top: -60px; /* HALF OF THE HEIGHT */
      margin-left: -300px; /* HALF OF THE WIDTH */
      border-radius: 25px;
      padding: 20px;
    }

    #snip-description {
      border-radius: 25px;
      border-width: 0px;
      width: inherit;
      padding: 10px;
    }

    #snip-tags {
      margin-top: 10px;
      border-radius: 25px;
      border-width: 0px;
      width: inherit;
      max-width: 80%;
      padding: 10px;
    }

    #snip-submit {
      margin-left: 10px;
      border-radius: 25px;
      padding: 8px;
      width: 16.5%;
      background-color: lightblue;
      border: 0px solid;
    }
  </style>

  <div id="add-snippet">
    <input id="snip-description" placeholder="Description", value="a description" />
    <input id="snip-tags" placeholder="Tags" value="tag1, tag2, tag3"/>
    <button id="snip-submit">Submit</button>
  </div>
      `;
  root.appendChild(div);

  chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
    if (hostElement.style.visibility === "hidden") {
      hostElement.style.visibility = "visible";
    } else {
      hostElement.style.visibility = "hidden";
    }
    return "success";
  });

  document.addEventListener("mouseup", async e => {
    let temp = window.getSelection();
    console.log("selection", temp.toString());
    await chrome.runtime.sendMessage(
      { status: "updateSelection", selection: temp.toString() },
      function (response) {}
    );
  });

  root.getElementById("snip-submit").addEventListener("click", async e => {
    let description = root.getElementById("snip-description").value;
    let tags = root.getElementById("snip-tags").value;
    await chrome.runtime.sendMessage(
      { status: "snip-submit", description: description, tags: tags },
      function (response) {}
    );
  });
}
