
function el_id_fileSelectorUploadButton() {
    const elem = getId("file-selector-upload-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const id_fileSelectorUploading =
                    getId("file-selector-uploading");
                const id_fileSelectorSelection =
                    getId("file-selector-selection");

                id_fileSelectorSelection.classList.add('is-hidden');
                id_fileSelectorUploading.classList.remove('is-hidden');
            }
        );
    }
}

//el_id_fileSelectorUploadButton();
