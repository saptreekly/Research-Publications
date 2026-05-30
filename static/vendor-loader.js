(function () {
  var scriptPromises = Object.create(null);

  function loadStylesheet(id, href) {
    if (document.getElementById(id)) {
      return Promise.resolve();
    }

    return new Promise(function (resolve, reject) {
      var link = document.createElement("link");
      link.id = id;
      link.rel = "stylesheet";
      link.href = href;
      link.crossOrigin = "anonymous";
      link.onload = function () {
        resolve();
      };
      link.onerror = reject;
      document.head.appendChild(link);
    });
  }

  function loadScript(id, src) {
    if (scriptPromises[id]) {
      return scriptPromises[id];
    }

    if (document.getElementById(id)) {
      scriptPromises[id] = Promise.resolve();
      return scriptPromises[id];
    }

    scriptPromises[id] = new Promise(function (resolve, reject) {
      var script = document.createElement("script");
      script.id = id;
      script.src = src;
      script.crossOrigin = "anonymous";
      script.onload = function () {
        resolve();
      };
      script.onerror = reject;
      document.head.appendChild(script);
    });

    return scriptPromises[id];
  }

  function resolveAsset(path) {
    var base = document.querySelector("base");
    var href = base && base.href ? base.href : window.location.href;
    return new URL(path.replace(/^\//, ""), href).toString();
  }

  window.ensureKaTeX = function () {
    if (window.__katexBundleReady) {
      return window.__katexBundleReady;
    }

    window.__katexBundleReady = loadStylesheet(
      "katex-css",
      "https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css"
    )
      .then(function () {
        return loadScript(
          "katex-js",
          "https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"
        );
      })
      .then(function () {
        return loadScript(
          "katex-auto-render",
          "https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/contrib/auto-render.min.js"
        );
      });

    return window.__katexBundleReady;
  };

  window.ensureLabPrism = function () {
    if (window.__labPrismReady) {
      return window.__labPrismReady;
    }

    window.__labPrismReady = loadScript(
      "prism-js",
      "https://cdn.jsdelivr.net/npm/prismjs@1.29.0/prism.min.js"
    ).then(function () {
      return loadScript(
        "prism-julia",
        "https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-julia.min.js"
      );
    });

    return window.__labPrismReady;
  };

  window.ensureLabEditor = function () {
    if (window.__labEditorReady) {
      return window.__labEditorReady;
    }

    window.__labEditorReady = window.ensureLabPrism().then(function () {
      return loadScript("lab-editor-js", resolveAsset("static/lab-editor.js"));
    });

    return window.__labEditorReady;
  };

  window.ensureLeaflet = function () {
    if (window.__leafletReady) {
      return window.__leafletReady;
    }

    window.__leafletReady = loadStylesheet(
      "leaflet-css",
      "https://cdn.jsdelivr.net/npm/leaflet@1.9.4/dist/leaflet.css"
    ).then(function () {
      return loadScript(
        "leaflet-js",
        "https://cdn.jsdelivr.net/npm/leaflet@1.9.4/dist/leaflet.js"
      );
    });

    return window.__leafletReady;
  };

  window.ensureSituationMap = function () {
    if (window.__situationMapReady) {
      return window.__situationMapReady;
    }

    window.__situationMapReady = window.ensureLeaflet().then(function () {
      return loadScript("situation-map-js", resolveAsset("static/situation-map.js"));
    });

    return window.__situationMapReady;
  };
})();
