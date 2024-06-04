/*
 *
 * main-page.js
 *
 * Copyright (c) 2023-2024 (authors)
 *
 * All rights reserved
 *
 * TALGS is distributed under the () license, see LICENSE for details
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
