(function () {
    function highlightTarget(textarea) {
        var shell = textarea.closest(".lab-code-editor-shell");
        if (!shell) {
            return;
        }

        var code = shell.querySelector(".lab-code-highlight code");
        if (!code || !window.Prism) {
            return;
        }

        code.textContent = textarea.value.endsWith("\n")
            ? textarea.value
            : textarea.value + "\n";
        window.Prism.highlightElement(code);
    }

    function syncScroll(textarea) {
        var shell = textarea.closest(".lab-code-editor-shell");
        if (!shell) {
            return;
        }

        var highlight = shell.querySelector(".lab-code-highlight");
        if (!highlight) {
            return;
        }

        highlight.scrollTop = textarea.scrollTop;
        highlight.scrollLeft = textarea.scrollLeft;
    }

    function whenPrismReady(callback) {
        if (window.Prism) {
            callback();
            return;
        }
        window.setTimeout(function () {
            whenPrismReady(callback);
        }, 40);
    }

    function bindEditor(textarea) {
        if (textarea.dataset.labEditorInit === "true") {
            highlightTarget(textarea);
            return;
        }

        textarea.dataset.labEditorInit = "true";
        textarea.addEventListener("input", function () {
            highlightTarget(textarea);
        });
        textarea.addEventListener("scroll", function () {
            syncScroll(textarea);
        });
        whenPrismReady(function () {
            highlightTarget(textarea);
        });
    }

    window.initLabCodeEditor = function (textarea) {
        if (!textarea) {
            return;
        }
        bindEditor(textarea);
    };

    window.refreshLabCodeEditor = function (textarea) {
        if (!textarea) {
            return;
        }
        highlightTarget(textarea);
    };

    if (document.documentElement) {
        var themeObserver = new MutationObserver(function () {
            document.querySelectorAll(".lab-code-editor[data-lab-editor-init='true']").forEach(function (textarea) {
                highlightTarget(textarea);
            });
        });
        themeObserver.observe(document.documentElement, {
            attributes: true,
            attributeFilter: ["data-theme"],
        });
    }
})();
