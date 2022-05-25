// #![allow(dead_code)]
pub mod a_star_search;
pub mod aa_tree;
pub mod ab_tree;
pub mod abelian_group;
pub mod absorbing_element;
pub mod abstract_structs;
pub mod accumulate;
pub mod adaptive_heap_sort;
pub mod additive_group;
pub mod addressable_heap;
pub mod adjacency_list_graph;
pub mod adjacency_list_to_directed_edges;
pub mod adjacency_matrix_graph;
pub mod af_heap;
pub mod aho_corasick;
pub mod all_pairs_shortest_path;
pub mod analysis;
pub mod analysis_search;
pub mod apsp_johnson_dense;
pub mod apsp_johnson_sparse;
pub mod arborescence;
pub mod argsort;
pub mod array_compression;
pub mod articulation_point;
pub mod associative_property;
pub mod automorphism;
pub mod auxiliary_tree;
pub mod avl_tree;
pub mod avl_tree_node;
pub mod avl_tree_tmp;
pub mod b_plus_tree;
pub mod b_star_tree;
pub mod balanced_tree;
pub mod ball_tree;
pub mod barrett_reduction;
pub mod batcher_odd_even_mergesort;
pub mod bead_sort;
pub mod beatty_sequence;
pub mod bellman_ford;
pub mod bellman_ford_dense;
pub mod bellman_ford_tree;
pub mod bidirected_graph;
pub mod bijection;
pub mod binary_gcd;
pub mod binary_heap;
pub mod binary_indexed_tree;
pub mod binary_operation;
pub mod binary_operation_id;
pub mod binary_relation;
pub mod binary_search;
pub mod binary_tree;
pub mod binary_tree_node;
pub mod binomial_heap;
pub mod bit_inverse;
pub mod bit_length;
pub mod bit_length_table;
pub mod bit_reverse;
pub mod bit_scan_forward;
pub mod bit_scan_reverse;
pub mod bit_shr_until_odd;
pub mod bitonic_sort;
pub mod bk_tree;
pub mod block_cut_tree;
pub mod block_sort;
pub mod borsuk_ulam;
pub mod breadth_first_search;
pub mod breadth_first_search_arborescence;
pub mod breadth_first_search_tree;
pub mod brent_cycle_detection;
pub mod bridge_finding;
pub mod bubble_sort;
pub mod bucket_sort;
pub mod bx_tree;
pub mod cache_oblivious_distributing_sort;
pub mod carmichael_number;
pub mod carmichael_theorem;
pub mod cartesian_tree;
pub mod cascade_merge_sort;
pub mod category;
pub mod category_theory;
pub mod centroid_decomposition;
pub mod chinese_remainder_theorem;
pub mod choose;
pub mod cht;
pub mod chu_liu_edmonds_minimum_spanning_arborescence;
pub mod cipolla_algorithm;
pub mod circle_dividing;
pub mod circle_hough_transform;
pub mod closest_pair_points;
pub mod cmp;
pub mod cocktail_shaker_sort;
pub mod comb_sort;
pub mod combination;
pub mod combination_choose;
pub mod combinatorics;
pub mod commutative_monoid;
pub mod commutative_property;
pub mod comparison_sort;
pub mod complete_digraph;
pub mod complete_graph;
pub mod connected_components;
pub mod connected_components_bfs;
pub mod connected_components_dfs;
pub mod connected_components_dsu;
pub mod convex_hull_trick;
pub mod count_leading_zeros;
pub mod count_trailing_zeros;
pub mod counting_sort;
pub mod cover_tree;
pub mod crt;
pub mod dag_lca;
pub mod dancing_link;
pub mod dancing_tree;
pub mod day_stout_warren;
pub(crate) mod debug_print;
pub mod decision_tree;
pub mod default;
pub mod deletion;
pub mod depth_first_search;
pub mod depth_first_search_arborescence;
pub mod depth_first_search_tree;
pub mod dial_sortest_path;
pub mod dijkstra_arborescence;
pub mod dijkstra_dense;
pub mod dijkstra_queue_binary_heap_std;
pub mod dijkstra_sparse_parents;
pub mod dijkstra_sparse_predecessors;
pub mod dijkstra_sparse_queue;
pub mod dijkstra_tree;
pub mod disjoint_set_union;
pub mod disjoint_sparse_table;
pub mod disjoint_sparse_table_range_get_query;
pub mod distributive_property;
pub mod division;
pub mod divisor;
pub mod doubly_chained_tree;
pub mod dsu;
pub mod dual_fenwick_tree;
pub mod dual_segment_tree;
pub mod dynamic_modulus;
pub mod dynamic_segment_tree;
pub mod dynamic_tensor_property;
pub mod dynamic_tree_lca_euler_tour;
pub mod edmonds_karp;
pub mod eppstein_algorithm;
pub mod euclidean_algorithms;
pub mod euler_totient;
pub mod euler_tour_edges;
pub mod euler_tour_ext;
pub mod euler_tour_nodes;
pub mod eulerian_circuit;
pub mod eulerian_path;
pub mod eulerian_trail;
pub mod exchange_sort;
pub mod exponential_tree;
pub mod extended_euclidean_algorithm;
pub mod extgcd_modinv;
pub mod factorial;
pub mod factorial_table;
pub mod fast_fourier_transform;
pub mod fast_mobius_transform;
pub mod fast_modulo_transform;
pub mod fast_zeta_transform;
pub mod fenwick_tree;
pub mod fenwick_tree_dual;
pub mod fft;
pub mod fibonacci_heap;
pub mod fibonacci_number;
pub mod field;
pub mod find_first_set;
pub mod finger_tree;
pub mod floor_sqrt;
pub mod floyd_warshall;
pub mod fold;
pub mod ford_fulkerson;
pub mod ford_johnson_algorithm;
pub mod formal_power_series;
pub mod fourier_transform;
pub mod fractal_tree_index;
pub mod frobenius_endmorphism;
pub mod gamma_function;
pub mod garner;
pub mod general_dijkstra_sparse;
pub mod genetic_algorithm;
pub mod geometry;
pub mod ghost_leg;
pub mod gnome_sort;
pub mod gradient_boostring;
pub mod graph;
pub mod graph_bfs;
pub mod graph_dfs;
pub mod graph_disconnected;
pub mod graph_edge_trait;
pub mod graph_edge_usize_usize_impl;
pub mod graph_edge_usize_usize_t_impl;
pub mod graph_edge_weight_impl;
pub mod graph_pointer_directed;
pub mod graph_pointer_mixed;
pub mod graph_pointer_undirected;
pub mod graph_trait_pointer_mixed;
pub mod greatest_common_divisor;
pub mod group;
pub mod group_theory;
pub mod group_theory_id;
pub mod group_theory_preset;
pub mod hash_tree;
pub mod heapsort;
pub mod heavy_light_decomposition;
pub mod height;
pub mod hilbert_r_tree;
pub mod histogram_sort;
pub mod homogeneous_product;
pub mod homomorphism;
pub mod hopcroft_karp;
pub mod idempotence;
pub mod identifier;
pub mod identity_element;
pub mod implicit_k_d_tree;
pub mod index;
pub mod insertion;
pub mod insertion_sort;
pub mod interpolation_sort;
pub mod inverse_element;
pub mod inverse_factorial_table;
pub mod io;
pub mod is_absorbing;
pub mod is_adjacency_matrix;
pub mod is_arborescence;
pub mod is_associative;
pub mod is_commutative;
pub mod is_distributive;
pub mod is_eulerian_graph;
pub mod is_idempotent;
pub mod is_identity;
pub mod is_invertible;
pub mod is_multitree;
pub mod is_pairwise_coprime;
pub mod is_polytree;
pub mod is_regular_graph;
pub mod is_setwise_coprime;
pub mod is_subsequence;
pub mod is_undirected_dense_graph;
pub mod is_zero_element;
pub mod isomorphism;
pub mod join;
pub mod k_d_tree;
pub mod karatsuba_algorithm;
pub mod knuth_morris_pratt;
pub mod las_vegas_algorithm;
pub mod lazy_segment_tree;
pub mod lazy_sqrt_decomposition;
pub mod lca_binary_lifting;
pub mod lca_eulertour_rmq;
pub mod lca_eulertour_rmq_disjoint_sparse_table;
pub mod lca_eulertour_rmq_segment_tree;
pub mod lca_eulertour_rmq_sparse_table;
pub mod lca_eulertour_rmq_sqrt_decomposition;
pub mod lca_hld;
pub mod least_common_multiple;
pub mod least_significant_bit;
pub mod least_significant_bit_number;
pub mod left_identity_element;
pub mod leftist_tree;
pub mod library_sort;
pub mod lightgbm;
pub mod linear_programming;
pub mod linear_sieve;
pub mod linear_time_minimum_spanning_tree;
pub mod link_cut_tree;
pub mod log_structured_merge_tree;
pub mod longest_common_prefix;
pub mod longest_common_prefix_kasai;
pub mod longest_increasing_sequence;
pub mod longest_palindromic_substring;
pub mod lowest_common_ancestor;
pub mod lowlink;
pub mod lsm_tree;
pub mod lucas_number;
pub mod lucas_sequence;
pub mod m_ary_tree;
pub mod maclaurin_series;
pub mod magma;
pub mod manacher;
pub mod matrix;
pub mod matrix_constant;
pub mod matrix_dynamic;
pub mod matrix_runtime_static;
pub mod matrix_static;
pub mod maximum_cardinality_matching;
pub mod merge_insertion_sort;
pub mod merge_sort;
pub mod merge_sort_inplace;
pub mod mergeable_heap;
pub mod mergesort;
pub mod merkle_tree;
pub mod metric_tree;
pub mod minimum_cost_arborescence;
pub mod minimum_cost_flow;
pub mod minimum_spanning_tree;
pub mod mo_algorithm;
pub mod mo_algorithm_3d;
pub mod mobius_transformation;
pub mod modular;
pub mod modular_constant;
pub mod modular_ext;
pub mod modular_power;
pub mod modular_runtime_static;
pub mod modular_static;
pub mod modular_tetration;
pub mod modulus;
pub mod monoid;
pub mod monte_carlo_algorithm;
pub mod morphism;
pub mod most_significant_bit;
pub mod most_significant_bit_number;
pub mod mst_boruvka;
pub mod mst_kruskal;
pub mod mst_prim_dense;
pub mod mst_prim_sparse;
pub mod mst_reverse_delete;
pub mod multi_key_quicksort;
pub mod multiplicative_inverse;
pub mod multiset;
pub mod n_group_category;
pub mod n_group_finite_group;
pub mod negative_cycle;
pub mod network_graph_node;
pub(crate) mod new_rc_refcell;
pub mod newton_method;
pub mod ntt;
pub mod number_theoritic_transform;
pub mod odd_even_sort;
pub mod offline_lca_tarjan;
pub mod order_static_tree;
pub mod ordered_set;
pub mod oscillating_merge_sort;
pub mod p_group;
pub mod pairing_heap;
pub mod pancacke_sorting;
pub mod parity_check_matrix;
pub mod partial_order;
pub mod pascal_rule;
pub mod pascal_simplex;
pub mod pascal_triangle;
pub mod patricia_tree;
pub mod persistent_union_find;
pub mod ph_tree;
pub mod pigeonhole_sort;
pub mod pivot_tree;
pub mod pointer_grpah;
pub mod pollard_p_1;
pub mod pollard_rho;
pub mod polyphase_merge_sort;
pub mod pop;
pub mod popcount;
pub mod popcount_table;
pub mod postman_sort;
pub mod potentialized_union_find;
pub mod power;
pub mod power_group;
pub mod power_monoid;
pub mod power_semigroup;
pub mod pq_tree;
pub mod pr_tree;
pub mod prefix_tree;
pub mod preorder;
pub mod primality;
pub mod primality_test_fermat;
pub mod primality_test_miller_rabin;
pub mod primality_test_rapin;
pub mod primality_test_solovay_strassen;
pub mod prime_counting_function;
pub mod prime_number;
pub mod priority_queue;
pub mod priority_queue_binary_heap_std_impl;
pub mod priority_r_tree;
pub mod proportion_extend_sort;
pub mod proxmap_sort;
pub mod prufer_group;
pub mod quasigroup;
pub mod quick_sort;
pub mod r_plus_tree;
pub mod r_star_tree;
pub mod rabin_karp;
pub mod radix_heap;
pub mod radix_sort;
pub mod radix_tree;
pub mod random_forest;
pub mod range_get_query;
pub mod range_minimum_query;
pub mod range_tree;
pub mod rectangle_tree;
pub mod red_black_tree;
pub mod reduce;
pub mod reflexive_relation;
pub mod rerooting;
pub mod reset_bit;
pub mod reset_least_bit;
pub mod right_identity_element;
pub mod ring;
pub mod rollback_union_find;
pub mod scapegoad_tree;
pub mod scapegoat_tree;
pub mod scc;
pub mod scc_kosaraju;
pub mod scc_path_based;
pub mod scc_tarjan_lowlink;
pub mod segment_tree;
pub mod segment_tree_2d;
pub mod segment_tree_2d_dense;
pub mod segment_tree_beats;
pub mod segment_tree_binary_search;
pub mod segment_tree_binary_search_recurse;
pub mod segment_tree_from_slice;
pub mod segment_tree_indexing;
pub mod segment_tree_range_get_query;
pub mod segment_tree_reduce_recurse;
pub mod selection_sort;
pub mod semigroup;
pub mod semiring;
pub mod set_theory;
pub mod shaker_sort;
pub mod shakutori_method;
pub mod shear_sort;
pub mod shellsort;
pub mod shortest_path;
pub mod shortest_path_arborescence;
pub mod shortest_path_potential;
pub mod shortest_path_predecessors;
pub mod shortest_path_tree;
pub mod shuffle_sort;
pub mod sieve_of_atkin;
pub mod sieve_of_eratosthenes;
pub mod simulated_annealing;
pub mod single_source_shortest_path;
pub mod size;
pub mod skew_heap;
pub mod sliding_window_aggregation;
pub mod slope_trick;
pub mod slowsort;
pub mod smallest_enclosing_circle;
pub mod smawk_algorithm;
pub mod smoothsort;
pub mod sort;
pub mod sorting_network;
pub mod sorting_number;
pub mod spaghetti_sort;
pub mod sparse_table;
pub mod sparse_table_range_get_query;
pub mod spfa;
pub mod splay_tree;
pub mod splay_tree_node;
pub mod split;
pub mod spqr_tree;
pub mod spreadsort;
pub mod sqrt_decomposition;
pub mod sqrt_decomposition_fast_reduce;
pub mod sqrt_decomposition_get_range_query;
pub mod sqrt_tree;
pub mod sssp_dijkstra_sparse;
pub mod sssp_faster_algorithm;
pub mod stable_sort;
pub mod static_modulus;
pub mod static_tensor_property;
pub mod steiner_tree;
pub mod stooge_sort;
pub mod string;
pub mod strongly_connected_components;
pub mod suffix_array;
pub mod suffix_array_doubling;
pub mod suffix_array_doubling_counting_sort;
pub mod suffix_array_sais;
pub mod suffix_tree;
pub mod swag;
pub mod t_tree;
pub mod tango_tree;
pub mod taylor_series;
pub mod tensor;
pub mod tensor_property;
pub mod ternary_heap;
pub mod ternary_search;
pub mod ternary_search_tree;
pub mod top_tree;
pub mod topological_sort;
pub mod topology;
pub mod torus;
pub mod total_order;
pub mod tournament_sort;
pub mod transitive_relation;
pub mod traveling_salesperson;
pub mod tree_bfs;
pub mod tree_depths;
pub mod tree_dfs;
pub mod tree_diameter;
pub mod tree_edges_to_graph;
pub mod tree_node;
pub mod tree_parents;
pub mod tree_path_query;
pub mod tree_path_query_binary_lifting;
pub mod tree_path_query_hld;
pub mod tree_sizes;
pub mod tree_sort;
pub mod tribonacci_number;
pub mod two_three_four_tree;
pub mod two_three_heap;
pub mod two_three_tree;
pub mod ub_tree;
pub mod undefined;
pub mod undirected_edges_to_directed;
pub mod union_find;
pub mod union_find_get_labels;
pub mod union_find_roots_are_same;
pub mod union_find_trait;
pub mod usize_min;
pub mod van_emde_boas_tree;
pub mod vector;
pub mod vector_util;
pub mod verbal_arithmetic;
pub mod virtual_tree;
pub mod vp_tree;
pub mod wavelet_matrix;
pub mod weighted_union_algorithm;
pub mod x_tree;
pub mod xor_sparse_table;
pub mod xorshift;
pub mod z_algorithm;
pub mod zero_element;
pub mod zero_one_bfs;
// pub mod rerooting_dynamic;
