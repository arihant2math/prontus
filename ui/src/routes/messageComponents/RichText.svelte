<script>
    import {rich} from "../api.js";

    export let content = "";
    let richContent = "";
    rich(content).then((data) => {
        console.log(data);
        richContent = data;
    });
</script>

{@html richContent}
