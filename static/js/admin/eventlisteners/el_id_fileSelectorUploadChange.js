
function el_id_fileSelectorUploadChange(function_value) {
    const elem = getId("file-selector-upload-change");

    const event_function = function() {
        const form_url = elem.dataset.formurl;

        const csrf_token_value = document.querySelector(
            '#file-upload-form input[name=csrf_token]'
        ).value;
        const file_id = document.querySelector(
            '#file-upload-form input[name=id]'
        ).value;
        const filename = document.querySelector(
            '#file-upload-form input[name=filename]'
        ).value;

        fetch(
            form_url,
            {
                method: 'POST',
                body: new URLSearchParams({
                    'csrf_token': csrf_token_value,
                    'id': file_id,
                    'filename': filename
                })
            }
        ).then(
            response => response.json()
        ).then(
            json_resp => {
                if (json_resp.success) {
                    const the_url = elem.dataset.url;
                    function_value(the_url);
                } else {
                    // TODO: Show error, etc.
                    console.error(error);
                }
            }
        ).catch(
            error => {
                // TODO: Show error, etc.
                console.error(error);
            }
        );
    };

    if (elem !== null) {
        // Button
        elem.addEventListener(
            'click',
            event_function
        );

        // Press enter on the input
        document.querySelector(
            '#file-upload-form input[name=filename]'
        ).addEventListener(
            'keyup',
            event_input => {
                // Number 13 is the "Enter" key on the keyboard
                if (event_input.keyCode === 13) {
                    event_function();
                }
            }
        );
    }
}

//el_id_fileSelectorUploadChange();
