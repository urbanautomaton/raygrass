use super::*;
use crate::material::LambertianMaterial;
use crate::texture::ConstantTexture;

type Subject = Sphere<'static, ConstantTexture>;

mod hit {
    use super::*;

    fn subject() -> Subject {
        Subject::new(
            Vec::new(0.0, 0.0, 0.0),
            1.0,
            ConstantTexture {
                color: Color::new(0.0, 0.0, 0.0),
            },
            &LambertianMaterial {},
        )
    }

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_at_the_center() {
        let ray = Ray {
            origin: Vec::new(-2.0, 0.0, 0.0),
            direction: Vec::new(1.0, 0.0, 0.0),
        };

        assert_eq!(subject().hit(&ray, 0.0, std::f64::INFINITY).unwrap().t, 1.0);
    }

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_from_the_other_direction() {
        let ray = Ray {
            origin: Vec::new(2.0, 0.0, 0.0),
            direction: Vec::new(-1.0, 0.0, 0.0),
        };

        assert_eq!(subject().hit(&ray, 0.0, std::f64::INFINITY).unwrap().t, 1.0);
    }

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_from_the_top() {
        let ray = Ray {
            origin: Vec::new(0.0, 2.0, 0.0),
            direction: Vec::new(0.0, -1.0, 0.0),
        };

        assert_eq!(subject().hit(&ray, 0.0, std::f64::INFINITY).unwrap().t, 1.0);
    }

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_from_the_diagonal() {
        let ray = Ray {
            origin: Vec::new(2.0, 2.0, 0.0),
            direction: Vec::new(-1.0, -1.0, 0.0).normalize(),
        };

        if let Some(hit) = subject().hit(&ray, 0.0, std::f64::INFINITY) {
            assert!((hit.t - 1.828).abs() <= 0.01);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_returns_none_for_a_ray_pointing_away_from_the_sphere() {
        let ray = Ray {
            origin: Vec::new(2.0, 0.0, 0.0),
            direction: Vec::new(1.0, 0.0, 0.0),
        };

        assert!(subject().hit(&ray, 0.0, std::f64::INFINITY).is_none());
    }
}
