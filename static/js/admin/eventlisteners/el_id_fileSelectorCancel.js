
function el_id_fileSelectorCancel() {
    const elem = getId("file-selector-cancel");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const id_fileInput = getId("file-input");
                id_fileInput.value = "0";

                const id_fileName = getId("file-name");
                id_fileName.style.display = "block";

                const id_fileImgcard = getId("file-imgcard");
                id_fileImgcard.style.display = "none";

                elem.classList.add('is-hidden');
            }
        );
    }
}

el_id_fileSelectorCancel();
