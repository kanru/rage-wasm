(ns app
  (:require ["@kanru/rage-wasm" :as rage]
            [promesa.core :as p]))

(defonce encoder
  (js/TextEncoder. "utf-8"))

(defonce decoder
  (js/TextDecoder. "utf-8"))

(defn encode [s]
  (.encode encoder s))

(defn decode [arr]
  (.decode decoder arr))

(defn x25519-encryption-and-decryption []
  (p/let [secret_key "AGE-SECRET-KEY-17QZPZDKGN49PAJHQGKDT056ND8MEZ6ZQK9HPXGMCS85VXNXEPATSQTYK6T"
          public_key "age1mfqmqkz9ga3a3lrgw8yatm79h5pqdu7z2hclghck5v8lrtwerysq97u6j8"
          data "test"
          encrypted (rage/encrypt_with_x25519 public_key (encode data) true)
          decrypted (rage/decrypt_with_x25519 secret_key encrypted)]
    (assert (= data (decode decrypted)))
    "ok"))

(defn x25519-keygen-and-encryption-and-decryption []
  (p/let [keys (rage/keygen)
          secret_key (first keys)
          public_key (second keys)
          data "test"
          encrypted (rage/encrypt_with_x25519 public_key (encode data) true)
          decrypted (rage/decrypt_with_x25519 secret_key encrypted)]
    (assert (= data (decode decrypted)))
    "ok"))

(defn x25519-keygen-and-unarmored-encryption-and-decryption []
  (p/let [keys (rage/keygen)
          secret_key (first keys)
          public_key (second keys)
          data "test"
          encrypted (rage/encrypt_with_x25519 public_key (encode data) false)
          decrypted (rage/decrypt_with_x25519 secret_key encrypted)]
    (assert (= data (decode decrypted)))
    "ok"))

(defn passphrase-encryption-and-decryption []
  (p/let [keys (rage/keygen)
          key "password"
          data "test"
          encrypted (rage/encrypt_with_user_passphrase key (encode data) true)
          decrypted (rage/decrypt_with_user_passphrase key encrypted)]
    (assert (= data (decode decrypted)))
    "ok"))

(defn passphrase-unarmored-encryption-and-decryption []
  (p/let [keys (rage/keygen)
          key "password"
          data "test"
          encrypted (rage/encrypt_with_user_passphrase key (encode data) false)
          decrypted (rage/decrypt_with_user_passphrase key encrypted)]
    (assert (= data (decode decrypted)))
    "ok"))


(defn init []
  (p/let [t1 (x25519-encryption-and-decryption)
          t2 (x25519-keygen-and-encryption-and-decryption)
          t3 (x25519-keygen-and-unarmored-encryption-and-decryption)
          t4 (passphrase-encryption-and-decryption)
          t5 (passphrase-unarmored-encryption-and-decryption)]
    (js/console.log "x25519-encryption-and-decryption..." t1)
    (js/console.log "x25519-keygen-and-encryption-and-decryption..." t2)
    (js/console.log "x25519-keygen-and-unarmored-encryption-and-decryption..." t3)
    (js/console.log "passphrase-encryption-and-decryption..." t4)
    (js/console.log "passphrase-unarmored-encryption-and-decryption..." t5)))