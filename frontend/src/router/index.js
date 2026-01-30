import { createRouter, createWebHistory } from "vue-router";
import Home from "../components/Home.vue";
import Login from "../components/Login.vue";

const routes = [
	{ path: "/home", component: Home },
	{ path: "/login", component: Login },
];

export default createRouter({
	history: createWebHistory(),
	routes,
});
