<script lang="ts">
	const formData = {
		email: '',
		username: '',
		password: ''
	};

    const cBase = 'container mx-auto bg-surface-600 p-4 space-y-4';
	const cForm = 'space-y-4';

    async function onFormSubmit() {
        await fetch(`http://0.0.0.0:8000/api/user/register`, {
            method: "POST",
            body: JSON.stringify(
                {
                    "email": formData.email,
                    "username": formData.username,
                    "password": formData.password,
                },
            ),
           	headers: {
    			'Content-Type': 'application/json'
            }}
        );

    }
</script>

<div class="{cBase}">
    <form class="modal-form {cForm}">
        <label class="label">
            <span>Email</span>
            <input class="input" type="email" bind:value={formData.email} />
        </label>
        <label class="label">
            <span>Username</span>
            <input class="input" type="text" bind:value={formData.username} />
        </label>
        <label class="label">
            <span>Password</span>
            <input class="input" type="password" bind:value={formData.password} />
        </label>
    </form>

    <button class="btn variant-filled space-y-4" on:click={onFormSubmit}>Register</button>
</div>
