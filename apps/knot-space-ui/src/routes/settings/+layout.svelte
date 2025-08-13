<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authStore } from '../../lib/stores';

  onMount(async () => {
    await authStore.initialize();
    
    const unsubscribe = authStore.subscribe(({ initialized, isAuthenticated }) => {
      if (initialized && !isAuthenticated) {
        goto('/login?redirectTo=/settings');
      }
    });

    // Check immediately after initialization
    if ($authStore.initialized && !$authStore.isAuthenticated) {
      goto('/login?redirectTo=/settings');
    }

    return unsubscribe;
  });
</script>

<slot />
