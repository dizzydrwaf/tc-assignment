<template>
    <div class="register-container">
        <h2>Register</h2>

        <form @submit.prevent="handleRegister">
            <div class="field">
                <label>Instans </label>
                <input
                type="instans"
                v-model="instans"
                required 
                placeholder="Instans"
            </div>

            <div class="field">
                <label>Förnamn </label>
                <input
                type="förnamn"
                v-model="förnamn"
                required
                placeholder="Förnamn"
                />
            </div>

            <div class="field">
                <label>Efternamn </label>
                <input
                type="efternamn"
                v-model="efternamn"
                required
                placeholder="Efternamn"
                />
            </div>

            <div class="field">
                <label>Email </label>
                <input
                type="email"
                v-model="email"
                required
                placeholder="you@example.com"
                />
            </div>

            <div class="field">
                <label>Password </label>
                <input
                type="password"
                v-model="password"
                required
                placeholder="••••••••"
                />
            </div>

        <button type="submit">Login</button>
        </form>

        <p v-if="error" class="error">{{ error }}</p>
    </div>
</template>

<script setup>
import { ref } from "vue";

const email = ref("");
const password = ref("");
const error = ref("");

/* NYI */
function _handleLogin() {
	error.value = "";

	// basic example validation
	if (!email.value || !password.value) {
		error.value = "Please fill in all fields";
		return;
	}

	// Replace this with real auth logic / API call
	if (email.value === "admin@example.com" && password.value === "password") {
		alert("Logged in!");
	} else {
		error.value = "Invalid credentials";
	}
}
</script>

<style scoped>
</style>
