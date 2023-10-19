<script lang="ts">
    import axios from 'axios';
    import { loggedIn, userId } from '$lib/store.js';

	const formData = {
		username: '',
		password: ''
	};

    const cBase = 'container mx-auto bg-surface-600 p-4 space-y-4';
	const cForm = 'space-y-4';

    async function onFormSubmit() {
        const baseUrl = 'http://0.0.0.0:8000';
        const resp = await axios({
            method: 'GET',
            url: `${baseUrl}/api/user/login`,
            params: {
                "username": formData.username,
                "password": formData.password,
            }
        }); 

        console.log(resp);

        loggedIn.set(true);
        userId.set(resp.data.id);
    }
</script>

<div class="{cBase}">
    <form class="modal-form {cForm}">
        <label class="label">
            <span>Username</span>
            <input class="input" type="text" bind:value={formData.username} />
        </label>
        <label class="label">
            <span>Password</span>
            <input class="input" type="password" bind:value={formData.password} />
        </label>
    </form>

    <button class="btn variant-filled space-y-4" on:click={onFormSubmit}>Login</button>
</div>
