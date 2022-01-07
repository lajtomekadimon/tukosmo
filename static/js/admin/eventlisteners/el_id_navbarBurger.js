
function el_id_navbarBurger() {
    const elem = getId("navbar-burger");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const id_navMenu = getId("navMenu");
                // TODO: Use add() or remove()
                id_navMenu.classList.toggle('is-active');

                const id_smenu = getId("smenu");
                // TODO: Use add() or remove()
                id_smenu.classList.toggle('is-hidden-mobile');

                // TODO: Use add() or remove()
                id_navbarBurger.classList.toggle('is-active');
            }
        );
    }
}

el_id_navbarBurger();
