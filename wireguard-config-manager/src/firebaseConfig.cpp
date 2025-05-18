// src/firebaseConfig.js
import { initializeApp } from "firebase/app";
// Optionally import other Firebase SDK functionalities:
// import { getAuth } from "firebase/auth";
// import { getFirestore } from "firebase/firestore";

const firebaseConfig = {
  apiKey: "YOUR_API_KEY",
  authDomain: "YOUR_PROJECT.firebaseapp.com",
  projectId: "YOUR_PROJECT",
  storageBucket: "YOUR_PROJECT.appspot.com",
  messagingSenderId: "YOUR_SENDER_ID",
  appId: "YOUR_APP_ID"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);

// Optionally initialize additional services:
// export const auth = getAuth(app);
// export const db = getFirestore(app);

export default app;