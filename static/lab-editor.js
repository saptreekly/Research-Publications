(function () {
    var INDENT = "    ";

    function highlightTarget(textarea) {
        var shell = textarea.closest(".lab-code-editor-shell");
        if (!shell) {
            return;
        }

        var code = shell.querySelector(".lab-code-highlight code");
        if (!code || !window.Prism || !window.Prism.languages.julia) {
            return;
        }

        code.className = "language-julia";
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

    function dispatchInput(textarea) {
        textarea.dispatchEvent(new Event("input", { bubbles: true }));
    }

    function whenPrismReady(callback) {
        if (window.Prism && window.Prism.languages.julia) {
            callback();
            return;
        }
        window.setTimeout(function () {
            whenPrismReady(callback);
        }, 40);
    }

    function lineRange(value, start, end) {
        var lineStart = value.lastIndexOf("\n", start - 1) + 1;
        var lineEnd = value.indexOf("\n", end);
        if (lineEnd === -1) {
            lineEnd = value.length;
        }
        return { lineStart: lineStart, lineEnd: lineEnd };
    }

    function indentBlock(textarea, dedent) {
        var value = textarea.value;
        var start = textarea.selectionStart;
        var end = textarea.selectionEnd;
        var range = lineRange(value, start, end);
        var block = value.slice(range.lineStart, range.lineEnd);
        var lines = block.split("\n");
        var changed = false;

        if (dedent) {
            lines = lines.map(function (line) {
                if (line.indexOf(INDENT) === 0) {
                    changed = true;
                    return line.slice(INDENT.length);
                }
                if (line.indexOf("\t") === 0) {
                    changed = true;
                    return line.slice(1);
                }
                return line;
            });
        } else {
            lines = lines.map(function (line) {
                changed = true;
                return INDENT + line;
            });
        }

        if (!changed) {
            return false;
        }

        var newBlock = lines.join("\n");
        textarea.value = value.slice(0, range.lineStart) + newBlock + value.slice(range.lineEnd);

        var leading = start - range.lineStart;
        var newStart = range.lineStart + leading + (dedent ? -Math.min(INDENT.length, leading) : INDENT.length);
        var delta = newBlock.length - block.length;
        textarea.selectionStart = Math.max(range.lineStart, newStart);
        textarea.selectionEnd = end + delta;
        return true;
    }

    function insertAtCursor(textarea, text) {
        var value = textarea.value;
        var start = textarea.selectionStart;
        var end = textarea.selectionEnd;
        textarea.value = value.slice(0, start) + text + value.slice(end);
        var pos = start + text.length;
        textarea.selectionStart = pos;
        textarea.selectionEnd = pos;
    }

    function currentLineIndent(value, pos) {
        var lineStart = value.lastIndexOf("\n", pos - 1) + 1;
        var lineEnd = value.indexOf("\n", pos);
        if (lineEnd === -1) {
            lineEnd = value.length;
        }
        var line = value.slice(lineStart, lineEnd);
        var match = line.match(/^[\t ]*/);
        return match ? match[0] : "";
    }

    function nextLineIndent(value, pos) {
        var lineStart = value.lastIndexOf("\n", pos - 1) + 1;
        var currentLine = value.slice(lineStart, pos);
        var base = currentLineIndent(value, pos);
        var trimmed = currentLine.trimEnd();

        if (
            /\b(function|if|elseif|else|for|while|try|begin|do|struct|module|macro|quote)\b[^#]*$/.test(
                trimmed
            ) ||
            /\(\s*$/.test(trimmed) ||
            /=\s*$/.test(trimmed)
        ) {
            return base + INDENT;
        }

        return base;
    }

    function handleKeyDown(event) {
        var textarea = event.target;
        if (!textarea.classList.contains("lab-code-editor")) {
            return;
        }

        if (event.key === "Tab") {
            event.preventDefault();
            var start = textarea.selectionStart;
            var end = textarea.selectionEnd;

            if (start !== end) {
                if (indentBlock(textarea, event.shiftKey)) {
                    dispatchInput(textarea);
                    highlightTarget(textarea);
                }
                return;
            }

            if (event.shiftKey) {
                var value = textarea.value;
                var lineStart = value.lastIndexOf("\n", start - 1) + 1;
                var before = value.slice(lineStart, start);
                if (before.endsWith(INDENT)) {
                    textarea.value =
                        value.slice(0, start - INDENT.length) + value.slice(start);
                    textarea.selectionStart = textarea.selectionEnd = start - INDENT.length;
                    dispatchInput(textarea);
                    highlightTarget(textarea);
                } else if (before.endsWith("\t")) {
                    textarea.value = value.slice(0, start - 1) + value.slice(start);
                    textarea.selectionStart = textarea.selectionEnd = start - 1;
                    dispatchInput(textarea);
                    highlightTarget(textarea);
                }
                return;
            }

            insertAtCursor(textarea, INDENT);
            dispatchInput(textarea);
            highlightTarget(textarea);
            return;
        }

        if (event.key === "Enter") {
            event.preventDefault();
            var value = textarea.value;
            var pos = textarea.selectionStart;
            var indent = nextLineIndent(value, pos);
            insertAtCursor(textarea, "\n" + indent);
            dispatchInput(textarea);
            highlightTarget(textarea);
            syncScroll(textarea);
            return;
        }

        if (event.key === "Backspace") {
            var start = textarea.selectionStart;
            var end = textarea.selectionEnd;
            if (start !== end) {
                return;
            }
            var value = textarea.value;
            if (value.slice(start - INDENT.length, start) === INDENT) {
                event.preventDefault();
                textarea.value = value.slice(0, start - INDENT.length) + value.slice(start);
                textarea.selectionStart = textarea.selectionEnd = start - INDENT.length;
                dispatchInput(textarea);
                highlightTarget(textarea);
            }
        }
    }

    function bindEditor(textarea) {
        if (textarea.dataset.labEditorInit === "true") {
            whenPrismReady(function () {
                highlightTarget(textarea);
            });
            return;
        }

        textarea.dataset.labEditorInit = "true";
        textarea.setAttribute("spellcheck", "false");
        textarea.setAttribute("autocorrect", "off");
        textarea.setAttribute("autocapitalize", "off");
        textarea.setAttribute("data-gramm", "false");
        textarea.setAttribute("data-enable-grammarly", "false");

        textarea.addEventListener("input", function () {
            highlightTarget(textarea);
        });
        textarea.addEventListener("scroll", function () {
            syncScroll(textarea);
        });
        textarea.addEventListener("keydown", handleKeyDown);

        whenPrismReady(function () {
            highlightTarget(textarea);
            window.requestAnimationFrame(function () {
                highlightTarget(textarea);
                syncScroll(textarea);
            });
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
        whenPrismReady(function () {
            highlightTarget(textarea);
            syncScroll(textarea);
        });
    };

    if (document.documentElement) {
        var themeObserver = new MutationObserver(function () {
            document
                .querySelectorAll(".lab-code-editor[data-lab-editor-init='true']")
                .forEach(function (textarea) {
                    highlightTarget(textarea);
                });
        });
        themeObserver.observe(document.documentElement, {
            attributes: true,
            attributeFilter: ["data-theme"],
        });
    }
})();
