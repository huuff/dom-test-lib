((rust-ts-mode
  (eglot-server-programs . (((rust-mode rust-ts-mode) "rust-analyzer" :initializationOptions
                             (:cargo (:features "all" :targets "all" :targetDir 't)
                                     :check (:command "clippy" :features "all")))))))

