import {computePosition, offset} from 'https://cdn.jsdelivr.net/npm/@floating-ui/dom@1.6.10/+esm';

export function positionPopovers() {
    document.querySelectorAll('[data-popover]').forEach((element) => {
        let target;
        if (element.dataset.hasOwnProperty("popoverTarget")) {
            target = document.getElementById(element.dataset.popoverTarget);
        } else if (element.dataset.hasOwnProperty("popoverTargetParent")) {
            target = element.parentElement
        }

        let position = element.dataset.popoverPosition || 'bottom';
        let flip = element.dataset.hasOwnProperty("popoverFlip");
        let offsetAmount = parseInt(element.dataset.popoverOffset) || 0;
        let crossOffsetAmount = parseInt(element.dataset.popoverCrossOffset) || null;

        if (target && element) {
            let offsetMiddleware;
            if (crossOffsetAmount !== null) {
                offsetMiddleware = offset(offsetAmount)
            } else {
                offsetMiddleware = offset({
                    mainAxis: offsetAmount,
                    crossAxis: crossOffsetAmount
                })
            }
            computePosition(target, element, {
                placement: position,
                middleware: [offsetMiddleware]
            }).then(({x, y}) => {
                Object.assign(element.style, {
                    left: `${x}px`,
                    top: `${y}px`,
                });
            });
        }

        let configure = element.dataset.hasOwnProperty("popoverConfigure");
        if (configure && !element.dataset.hasOwnProperty("popoverConfigured")) {
            if (element.dataset.hasOwnProperty("popoverRef")) {
                let ref = element.dataset.popoverRef;
                target = element.parentElement.querySelector(`[data-popover-ref-target="${ref}"]`);
            }
            let showMethod = element.dataset.popoverShowMethod || "click";

            if (showMethod === "click") {
                target.onclick = () => {
                    element.classList.toggle("hidden");
                };
                document.body.addEventListener('click', function (event) {
                    if (!element.classList.contains("hidden") && !element.contains(event.target) && !target.contains(event.target)) {
                        element.classList.add("hidden");
                    }
                });
            } else if (showMethod === "hover") {
                target.addEventListener("mouseover", () => {
                    element.classList.remove("hidden");
                });
                target.addEventListener("mouseleave", () => {
                    element.classList.add("hidden");
                });
            }
            element.dataset.popoverConfigured = "true";
        }
    });
}

export function initPopovers() {
    positionPopovers();

    window.addEventListener('resize', positionPopovers);
    window.addEventListener('scroll', positionPopovers);
}
