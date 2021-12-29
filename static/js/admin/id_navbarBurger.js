
const id_navbarBurger = document.getElementById("navbar-burger");

if (id_navbarBurger !== null) {
    id_navbarBurger.addEventListener(
        'click',
        function() {
            const id_navMenu = document.getElementById("navMenu");
            id_navMenu.classList.toggle('is-active');

            const id_smenu = document.getElementById("smenu");
            id_smenu.classList.toggle('is-hidden-mobile');

            id_navbarBurger.classList.toggle('is-active');
        }
    );
}

