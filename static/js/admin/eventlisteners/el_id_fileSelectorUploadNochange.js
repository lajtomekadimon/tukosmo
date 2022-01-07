
function el_id_fileSelectorUploadNochange(function_value) {
    const elem = getId("file-selector-upload-nochange");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const the_url = event.currentTarget.dataset.url;
                function_value(the_url);
            }
        );
    }
}

//el_id_fileSelectorUploadNochange();
