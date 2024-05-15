(ns factors.main)

(defn sift "Keep elements of one list based on the elements of another."
  [v f]
  (map first (filter (fn [[_ bool]] bool) (map vector v f))))

(defn int-sqrt "Return the integer closest to the square root of a number 
                without going over."
  ([n] (int-sqrt 1 n))
  ([n m] (if (> (* (+ n 1) (+ n 1)) m)
          n
          (int-sqrt (+ n 1) m))))

(defn factor "Return all the factors of a number."
  [n]
  (let
   [u (range 1 (+ 1 (int-sqrt n)))]
   (map #(vector % (/ n %)) (sift u (map #(== 0 (mod n %)) u)))))

(defn -main [] (doseq [
                       l
                       (map 
                        #(str (first %) " x " (last %)) 
                        (factor 16))] 
                 (println l)))

(factor 16)

;; (-main)
