;; use real id
(hash 'client_id 123456789012345678
      
      ;; out of 4: plyaing, watching, streaming, competing
      'activity_type "playing"
      
      'state "gazing into the void"
      'details "rusted over"
      'instance #t
      
      'timestamps (hash 'start 1718923200)
      ;; optinal, could make a progress bar
      ;; 'end 1718926800
      
      'assets (hash ;; image names should be taken from developer portal
                    ;; urls and text are optional
                    'large_image "big"
                    'large_text ""
                    'large_url ""
                    
                    'small_image "small")
                    ;; 'small_text ""
                    ;; 'small_url ""
      
      ;; literal party lule
      'party (hash 'id "party"
                   'size '(1 4))
      
      'secrets (hash 'join "join_secret_string"
                     'spectate "spectate_secret_string"
                     'match "match_secret_string")
      
      ;; could be up to 2
      ;; optinal as heck
      ;; label - 32 chars max
      ;; url - 512 chars max
      ;; 'buttons (list (hash 'label ""
      ;;                      'url ""))
      )
