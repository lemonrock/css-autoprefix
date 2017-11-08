# css-autoprefix

[css-autoprefix] is a rust crate that provides a library similar to [autoprefixer](https://github.com/postcss/autoprefixer), but in Rust.

Currently needs nightly rust as a dependency (phf) needs nightly.


## The following property values are not autoprefixed


			// Property Values
			/*
				Feature Name			Property Name(s)	Property Values
				intrinsic-width			'width', 'min-width', 'max-width', 'height', 'min-height', 'max-height', 'inline-size', 'min-inline-size', 'max-inline-size', 'block-size', 'min-block-size', 'max-block-size', 'grid', 'grid-template', 'grid-template-rows', 'grid-template-columns', 'grid-auto-columns', 'grid-auto-rows'    with values   'max-content', 'min-content', 'fit-content', 'fill', 'fill-available', 'stretch'
				css-filter-function		'background', 'background-image', 'border-image', 'mask', 'list-style', 'list-style-image', 'content', 'mask-image'     filter() (a filter function, needs prefixing with -webkit- currently)
				css-element-function	'background', 'background-image', 'border-image', 'mask', 'list-style', 'list-style-image', 'content', 'mask-image'		element() (a element function, needs prefixing with -moz- currently)
				css-image-set	'background', 'background-image', 'border-image', 'mask', 'list-style', 'list-style-image', 'content', 'mask-image'		image-set() (an image-set function, needs prefixing)
				css-cross-fade	'background', 'background-image', 'border-image', 'mask', 'list-style', 'list-style-image', 'content', 'mask-image'		cross-fade() (an image-set function, needs prefixing)
				css-unicode-bidi		unicode-bidi	Various settings for property values. Quite messy. Not widely used. Note that `css-unicode-bidi` is the name autoprefixer uses, but it does not seem to be in my caniuse.com database.
				css-gradients (Historic)
				calc (Historic); a function that affects all properties
				css3-cursors-newer (Historic)  cursor	zoom-in,zoom-out
				css-crisp-edges: too messy; Chrome and Firefox conflict
			*/

## Licensing

The license for this project is MIT.

[css-autoprefix]: https://github.com/lemonrock/css-autoprefix "css-autoprefix GitHub page"
