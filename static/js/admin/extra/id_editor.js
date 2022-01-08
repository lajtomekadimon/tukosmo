
const id_editor = document.getElementById("editor");

if (id_editor !== null) {
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

        // Toolbar items
        /*
        toolbarItems: [
            ['heading', 'bold', 'italic', 'strike'],
            ['hr', 'quote'],
            ['ul', 'ol', 'task', 'indent', 'outdent'],
            ['table', 'image', 'link'],
            ['code', 'codeblock'],
        ],
        */

        hooks: {
            addImageBlobHook: (blob, callback) => {
                let formData = new FormData();

                // file in a 'multipart/form-data' request
                formData.append(0, blob, blob.name);

                fetch(
                    // TODO: Don't use a string, import the route!
                    '/en/admin/json/upload_file',
                    {
                        method: 'POST',
                        body: formData
                    }
                ).then(
                    response => {
                        if (response.ok) {
                            return response.json();
                        }

                        throw new Error('Server or network error');
                    }
                ).then(
                    json_resp => {
                        if (json_resp.success) {
                            callback(json_resp.url, json_resp.filename);
                        } else {
                            console.error("Something went bad.");
                            // TODO: Show error, etc.
                        }
                    }
                ).catch(
                    error => {
                        // TODO: Show error, etc.
                        console.error(error);
                    }
                );
            },
        },

        // Editor's initial value
        initialValue: body_text_value
    });

    /*
    editor.insertToolbarItem(
        {
            groupIndex: 3,
            itemIndex: 1
        },
        {
            name: 'myItem',
            tooltip: 'Insert image',
            command: 'image',
            //text: '@',
            className: 'image toastui-editor-toolbar-icons first'
        }
    );
    */
}

