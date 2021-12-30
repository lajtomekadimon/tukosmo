
function loadFileSelector(url) {
    var http = new XMLHttpRequest();
    http.open('GET', url, true);
    http.responseType = 'json';

    http.onreadystatechange = function() {
        if (http.readyState == 4 && http.status == 200) {
            if (http.response.success) {
                const id_fileSelectorPanel =
                    document.getElementById("file-selector-panel");
                id_fileSelectorPanel.innerHTML = http.response.html;

                const id_fileInput = document.getElementById("file-input");
                const id_fileName = document.getElementById("file-name");
                const id_fileImg = document.getElementById("file-img");
                const id_fileImgcard = document.getElementById("file-imgcard");

                const classes_fileSelectorPage =
                    document.querySelectorAll('.file-selector-page');
                classes_fileSelectorPage.forEach(
                    el => el.addEventListener(
                        'click',
                        event => {
                            const new_route =
                                event.currentTarget.dataset.route;
                            loadFileSelector(new_route);
                        }
                    )
                );

                const classes_selectFile =
                    document.querySelectorAll('.select-file');
                classes_selectFile.forEach(
                    el => el.addEventListener(
                        'click',
                        event => {
                            const file_id = event.currentTarget.dataset.id;
                            const file_name = event.currentTarget.dataset.name;
                            const file_route =
                                event.currentTarget.dataset.route;

                            id_fileInput.value = file_id;

                            id_fileName.style.display = "none";

                            id_fileImg.src = file_route;
                            id_fileImg.alt = file_name;

                            id_fileImgcard.style.display = "block";

                            id_fileSelectorPanel.classList.toggle('is-active');
                        }
                    )
                );

                const classes_fileSelectorClose =
                    document.querySelectorAll('.file-selector-close');
                classes_fileSelectorClose.forEach(
                    el => el.addEventListener(
                        'click',
                        event => {
                            id_fileSelectorPanel.classList.toggle('is-active');
                        }
                    )
                );

                const id_fileSelectorCancel =
                    document.getElementById("file-selector-cancel");
                id_fileSelectorCancel.addEventListener(
                    'click',
                    function() {
                        id_fileInput.value = "0";

                        id_fileName.style.display = "block";

                        id_fileImgcard.style.display = "none";

                        id_fileSelectorPanel.classList.toggle('is-active');
                    }
                );
            }
        }
    }

    http.send();
}


const id_fileSelectorJs = document.getElementById("file-selector-js");

if (id_fileSelectorJs !== null) {
    id_fileSelectorJs.addEventListener(
        'click',
        function() {
            const url = id_fileSelectorJs.dataset.url;

            loadFileSelector(url);

            const id_fileSelectorPanel =
                document.getElementById("file-selector-panel");
            id_fileSelectorPanel.classList.toggle('is-active');
        }
    );
}

