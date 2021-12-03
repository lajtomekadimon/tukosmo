
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


const delete_notifs = document.querySelectorAll('.delete');

delete_notifs.forEach(el => el.addEventListener('click', event => {
    event.target.parentNode.style.display = 'none';
}));

