<script lang="ts">
  // TODO: Implement historical view
  import Historical from "./Historical.svelte";
  import InProgress from "./InProgress.svelte";
  import { chainInfo } from "../../store.ts";
  let vote_in_progress = false;
  let vote_window_expired = false;

  chainInfo.subscribe((info_str) => {
    let data = JSON.parse(info_str);
    // TODO: find a better way to check the data is ready
    if (data.chain_view && data.chain_view.upgrade) {
      vote_in_progress = data.chain_view.upgrade.upgrade.validators_voted.length > 0;
    }
  });
</script>

<main>
  <div class="uk-child-width-expand@s" uk-grid>
    <div class="uk-grid-item-match">
      <div class="uk-card uk-card-default uk-card-body">
        {#if vote_in_progress}
          <InProgress />
        {:else}
          <div class="uk-text-center">
            <h4 class="uk-text-uppercase uk-text-muted">
              No Current Upgrade Proposals
            </h4>
          </div>
        {/if}

        {#if vote_window_expired}
          <div>
            <h3>Expired Proposals</h3>
            <InProgress />
          </div>
        {/if}
      </div>
    </div>
    <!-- <div>
      TODO: Implement this
      <Historical />
    </div> -->
  </div>
</main>
