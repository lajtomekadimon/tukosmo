
function loadFileSelector(url) {
    var http = new XMLHttpRequest();
    http.open('GET', url, true);
    http.responseType = 'json';

    http.onreadystatechange = function() {
        if (http.readyState == 4 && http.status == 200) {
            if (http.response.success) {
                const id_fileSelectorPanel = getId("file-selector-panel");
                id_fileSelectorPanel.innerHTML = http.response.html;

                el_classes_fileSelectorPage(loadFileSelector);

                el_classes_selectFile();

                el_classes_fileSelectorClose();

                el_id_fileSelectorCancel();

                el_id_fileSelectorUploadButton();
                el_id_fileSelectorSelectButton(loadFileSelector);

                el_classes_notification();
                el_classes_delete();

                el_id_fileSelectorUploadChange(loadFileSelector);
                el_id_fileSelectorUploadNochange(loadFileSelector);

                el_id_fileUpload_input();
            }
        }
    }

    http.send();
}


function el_id_fileSelectorJs() {
    const elem = getId("file-selector-js");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const url = elem.dataset.url;

                loadFileSelector(url);

                const id_fileSelectorPanel = getId("file-selector-panel");
                id_fileSelectorPanel.classList.add('is-active');
            }
        );
    }
}

el_id_fileSelectorJs();

