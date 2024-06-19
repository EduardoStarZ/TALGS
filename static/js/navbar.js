/*
 *
 * navbar.js
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */


$(document).ready(function() {
    /* Navbar scroll detection */

    $(function () {
		var lastScrollTop = 0, delta = 5;
		$(window).scroll(function () {
			var nowScrollTop = $(this).scrollTop();
			if (Math.abs(lastScrollTop - nowScrollTop) >= delta) {
				if (nowScrollTop > lastScrollTop) {
					$("#navbar").removeClass("is-fixed-top");
					$("html").removeClass("has-navbar-fixed-top");
				} else {
					$("#navbar").addClass("is-fixed-top");
					$("html").addClass("has-navbar-fixed-top");
				}
				lastScrollTop = nowScrollTop;
			}
		});
	});



    // Check for click events on the navbar burger icon
    $(".navbar-burger").click(function() {
        
        // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
        $(".navbar-burger").toggleClass("is-active");
        $(".navbar-menu").toggleClass("is-active");
        
    });


});
