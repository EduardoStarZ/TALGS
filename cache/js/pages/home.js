$(document).ready(function() {

    const dialog = document.querySelector("dialog");

    $("#login").click(function() {
        dialog.showModal();
        
    });

    $("#closeButton").click(function() {
        dialog.close();
    })

    $("#topo").click(function () {
        $("html, body").animate({ scrollTop: 0 }, 1000);
    });
})
