/*
 *
 * main-page.js
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

htmx.onLoad(function(content) {
		$('#new-request').click(function() {
				$('#go-back').removeClass('hidden');
		})

		$("#go-back").click(function() {
				$(this).addClass('hidden');
				htmx.ajax('GET', '/api/sale/client', "#master-frame");
		})
})
