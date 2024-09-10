import {computePosition, offset} from 'https://cdn.jsdelivr.net/npm/@floating-ui/dom@1.6.10/+esm';

export function positionPopovers() {
    console.debug('positionPopovers called');
    document.querySelectorAll('[data-popover]').forEach((element) => {
        let target = document.getElementById(element.dataset.popoverTarget);

        let position = element.dataset.popoverPosition || 'bottom';
        let flip = element.dataset.hasOwnProperty("popoverFlip");
        let offsetAmount = parseInt(element.dataset.popoverOffset) || 0;

        if (target && element) {
            computePosition(target, element, {
                placement: position,
                middleware: [offset(offsetAmount)]
            }).then(({x, y}) => {
                Object.assign(element.style, {
                    left: `${x}px`,
                    top: `${y}px`,
                });
            });
        }
    });
}

export function initPopovers() {
    positionPopovers();

    window.addEventListener('resize', positionPopovers);
    window.addEventListener('scroll', positionPopovers);
}
