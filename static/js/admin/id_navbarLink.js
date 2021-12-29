
const id_navbarLink = document.getElementById("navbar-link");

if (id_navbarLink !== null) {
    id_navbarLink.addEventListener(
        'click',
        function() {
            const id_navbarDropdown =
                document.getElementById("navbar-dropdown");
            id_navbarDropdown.classList.toggle('is-active');
        }
    );
}

