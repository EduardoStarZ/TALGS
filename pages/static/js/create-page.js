/*
 *
 * craete-page.js
 *
 * Copyright (c) 2023-2024 (authors)
 *
 * All rights reserved
 *
 * TALGS is distributed under the () license, see LICENSE for details
 * 
 * */

htmx.onLoad(function(content) {
		$(".has-consequences").click(function(e){
				let element_id = $(this).attr('id');

				let splitter = element_id.split('-')

				let buffer = splitter[0];

				switch(buffer) {
						case "available": {
								$(`#selected-card-${splitter[2]}`).removeClass("hidden");
								$(`#available-card-${splitter[2]}`).addClass("hidden");
						}
								break;
						case "selected": {
								$(`#available-card-${splitter[2]}`).removeClass("hidden");
								$(`#selected-card-${splitter[2]}`).remove();
						}
								break;
				}
		})

})
