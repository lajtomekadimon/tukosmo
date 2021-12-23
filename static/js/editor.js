
const Editor = toastui.Editor;

const body_text_value = document.getElementById("body-text").value;

const language_of_user = document.getElementsByTagName('html')[0]
    .getAttribute('lang');

// Full list of options:
// https://nhn.github.io/tui.editor/latest/ToastUIEditorCore
const editor = new Editor({
    // Container element
    el: document.querySelector('#editor'),

    // Language
    language: language_of_user,

    // Editor's height style value. Height is applied as border-box
    // (examples: '300px', '100%', 'auto', ...)
    height: '500px',

    // Initial editor type (markdown, wysiwyg)
    initialEditType: 'markdown',

    // Markdown editor's preview style (tab, vertical)
    previewStyle: 'vertical',

    // Send hostname to Google Analytics
    usageStatistics: false,

    // Events
    events: {
        change: function() {
            const body_text = document.getElementById("body-text");
            body_text.value = editor.getMarkdown();
        },
    },

    // Editor's initial value
    initialValue: body_text_value
});

editor.getMarkdown();
