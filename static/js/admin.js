
let dropdown_lang_button = document.getElementById("dropdown-lang-button");

if (dropdown_lang_button !== null) {
    dropdown_lang_button.addEventListener('click', function() {
        document.getElementById("dropdown-lang").classList.toggle('is-active');
    });
}


let navbar_burger_button = document.getElementById("navbar-burger");

if (navbar_burger_button !== null) {
    navbar_burger_button.addEventListener('click', function() {
        document.getElementById("navMenu").classList.toggle('is-active');
        document.getElementById("smenu").classList.toggle('is-hidden-mobile');
        document.getElementById("navbar-burger").classList.toggle('is-active');
    });
}


let navbar_dropdown_button = document.getElementById("navbar-link");

if (navbar_dropdown_button !== null) {
    navbar_dropdown_button.addEventListener('click', function() {
        let navbar_dropdown = document.getElementById("navbar-dropdown");
        navbar_dropdown.classList.toggle('is-active');
    });
}


let fileInput = document.querySelector('#file-js input[type=file]');

if (fileInput !== null) {
    fileInput.onchange = () => {
        if (fileInput.files.length > 0) {
            const fileName = document.querySelector('#file-js .file-name');
            fileName.textContent = fileInput.files[0].name;
        }
    }
}


function loadFileSelector(url) {
    var http = new XMLHttpRequest();
    http.open('GET', url, true);
    http.responseType = 'json';

    http.onreadystatechange = function() {
        if (http.readyState == 4 && http.status == 200) {
            let fileSelectorPanel = document.getElementById("file-selector-panel");
            if (http.response.success) {
                fileSelectorPanel.innerHTML = http.response.html;

                document.querySelectorAll('.file_selector_page').forEach(
                    el => el.addEventListener(
                        'click',
                        event => {
                            loadFileSelector(event.currentTarget.dataset.route);
                        }
                    )
                );

                document.querySelectorAll('.select_file').forEach(
                    el => el.addEventListener(
                        'click',
                        event => {
                            let file_id = event.currentTarget.dataset.id;
                            let file_name = event.currentTarget.dataset.name;
                            let file_route = event.currentTarget.dataset.route;

                            document.getElementById("file_input").value = file_id;

                            let fileName = document.getElementById("file_name");
                            fileName.style.display = "none";

                            let fileImg = document.getElementById("file_img");
                            fileImg.src = file_route;
                            fileImg.alt = file_name;
                            document.getElementById("file_imgcard").style.display = "block";

                            fileSelectorPanel.classList.toggle('is-active');
                        }
                    )
                );

                document.querySelectorAll('.file-selector-close').forEach(
                    el => el.addEventListener(
                        'click',
                        event => {
                            fileSelectorPanel.classList.toggle('is-active');
                        }
                    )
                );

                document.getElementById("file_selector_cancel").addEventListener('click', function() {
                    document.getElementById("file_input").value = "0";

                    let fileName = document.getElementById("file_name");
                    fileName.style.display = "block";

                    document.getElementById("file_imgcard").style.display = "none";

                    fileSelectorPanel.classList.toggle('is-active');
                });
            }
        }
    }

    http.send();
}


let fileSelector = document.getElementById("file-selector-js");

if (fileSelector !== null) {
    fileSelector.addEventListener('click', function() {

        var url = fileSelector.dataset.url;

        loadFileSelector(url);

        let fileSelectorPanel = document.getElementById("file-selector-panel");
        fileSelectorPanel.classList.toggle('is-active');

    });
}


document.querySelectorAll('.delete').forEach(
    el => el.addEventListener(
        'click',
        event => {
            event.currentTarget.parentNode.style.opacity = '0';
        }
    )
);

document.querySelectorAll('.notification').forEach(
    el => el.addEventListener(
        'transitionend',
        event => {
            event.currentTarget.remove();
        }
    )
);
