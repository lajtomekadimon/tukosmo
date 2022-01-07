
function el_id_fileSelectorSelectButton(function_value) {
    const elem = getId("file-selector-select-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const the_url = elem.dataset.url;
                function_value(the_url);
            }
        );
    }
}

//el_id_fileSelectorSelectButton();
