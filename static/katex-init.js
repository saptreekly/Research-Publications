window.renderMarkdownMath = function (element) {
  if (!element) {
    return;
  }

  var render = function () {
    if (typeof renderMathInElement !== "function") {
      return;
    }

    requestAnimationFrame(function () {
      renderMathInElement(element, {
        delimiters: [
          { left: "$$", right: "$$", display: true },
          { left: "$", right: "$", display: false },
          { left: "\\[", right: "\\]", display: true },
          { left: "\\(", right: "\\)", display: false },
        ],
        throwOnError: false,
        ignoredTags: ["script", "noscript", "style", "textarea", "pre", "code"],
      });
    });
  };

  if (typeof window.ensureKaTeX === "function") {
    window.ensureKaTeX().then(render);
    return;
  }

  render();
};
