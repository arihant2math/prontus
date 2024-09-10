<script>
    import {rich} from "../api.js";

    export let content = "";
    let richContent = "";
    rich(content).then((data) => {
        richContent = data;
    });
</script>

{@html richContent}
