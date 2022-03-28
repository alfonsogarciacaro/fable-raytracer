class Vector {
    constructor(x, y, z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }
    static times(k, v) {
        return new Vector(k * v.x, k * v.y, k * v.z);
    }
    static minus(v1, v2) {
        return new Vector(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z);
    }
    static plus(v1, v2) {
        return new Vector(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);
    }
    static dot(v1, v2) {
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    }
    static mag(v) {
        return Math.sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
    }
    static norm(v) {
        let mag = Vector.mag(v);
        let div = (mag === 0) ? Infinity : 1.0 / mag;
        return Vector.times(div, v);
    }
    static cross(v1, v2) {
        return new Vector(v1.y * v2.z - v1.z * v2.y, v1.z * v2.x - v1.x * v2.z, v1.x * v2.y - v1.y * v2.x);
    }
}
class Color {
    constructor(r, g, b) {
        this.r = r;
        this.g = g;
        this.b = b;
    }
    static scale(k, v) {
        return new Color(k * v.r, k * v.g, k * v.b);
    }
    static plus(v1, v2) {
        return new Color(v1.r + v2.r, v1.g + v2.g, v1.b + v2.b);
    }
    static times(v1, v2) {
        return new Color(v1.r * v2.r, v1.g * v2.g, v1.b * v2.b);
    }
    static toDrawingColor(c) {
        let legalize = (d) => d > 1 ? 1 : d;
        return {
            r: Math.floor(legalize(c.r) * 255),
            g: Math.floor(legalize(c.g) * 255),
            b: Math.floor(legalize(c.b) * 255)
        };
    }
}
Color.white = new Color(1.0, 1.0, 1.0);
Color.grey = new Color(0.5, 0.5, 0.5);
Color.black = new Color(0.0, 0.0, 0.0);
Color.background = Color.black;
Color.defaultColor = Color.black;
class Camera {
    constructor(pos, lookAt) {
        this.pos = pos;
        let down = new Vector(0.0, -1.0, 0.0);
        this.forward = Vector.norm(Vector.minus(lookAt, this.pos));
        this.right = Vector.times(1.5, Vector.norm(Vector.cross(this.forward, down)));
        this.up = Vector.times(1.5, Vector.norm(Vector.cross(this.forward, this.right)));
    }
}
class Sphere {
    constructor(center, radius, surface) {
        this.center = center;
        this.surface = surface;
        this.radius2 = radius * radius;
    }
    normal(pos) {
        return Vector.norm(Vector.minus(pos, this.center));
    }
    intersect(ray) {
        let eo = Vector.minus(this.center, ray.start);
        let v = Vector.dot(eo, ray.dir);
        let dist = 0;
        if (v >= 0) {
            let disc = this.radius2 - (Vector.dot(eo, eo) - v * v);
            if (disc >= 0) {
                dist = v - Math.sqrt(disc);
            }
        }
        if (dist === 0) {
            return null;
        }
        else {
            return { thing: this, ray: ray, dist: dist };
        }
    }
}
class Plane {
    constructor(norm, offset, surface) {
        this.surface = surface;
        this.normal = function (pos) { return norm; };
        this.intersect = function (ray) {
            let denom = Vector.dot(norm, ray.dir);
            if (denom > 0) {
                return null;
            }
            else {
                let dist = (Vector.dot(norm, ray.start) + offset) / (-denom);
                return { thing: this, ray: ray, dist: dist };
            }
        };
    }
}
var Surfaces;
(function (Surfaces) {
    Surfaces.shiny = {
        diffuse: function (pos) { return Color.white; },
        specular: function (pos) { return Color.grey; },
        reflect: function (pos) { return 0.7; },
        roughness: 250
    };
    Surfaces.checkerboard = {
        diffuse: function (pos) {
            if ((Math.floor(pos.z) + Math.floor(pos.x)) % 2 !== 0) {
                return Color.white;
            }
            else {
                return Color.black;
            }
        },
        specular: function (pos) { return Color.white; },
        reflect: function (pos) {
            if ((Math.floor(pos.z) + Math.floor(pos.x)) % 2 !== 0) {
                return 0.1;
            }
            else {
                return 0.7;
            }
        },
        roughness: 150
    };
})(Surfaces || (Surfaces = {}));
class RayTracer {
    constructor() {
        this.maxDepth = 5;
    }
    intersections(ray, scene) {
        let closest = +Infinity;
        let closestInter = undefined;
        for (let i in scene.things) {
            let inter = scene.things[i].intersect(ray);
            if (inter != null && inter.dist < closest) {
                closestInter = inter;
                closest = inter.dist;
            }
        }
        return closestInter;
    }
    testRay(ray, scene) {
        let isect = this.intersections(ray, scene);
        if (isect != null) {
            return isect.dist;
        }
        else {
            return undefined;
        }
    }
    traceRay(ray, scene, depth) {
        let isect = this.intersections(ray, scene);
        if (isect === undefined) {
            return Color.background;
        }
        else {
            return this.shade(isect, scene, depth);
        }
    }
    shade(isect, scene, depth) {
        let d = isect.ray.dir;
        let pos = Vector.plus(Vector.times(isect.dist, d), isect.ray.start);
        let normal = isect.thing.normal(pos);
        let reflectDir = Vector.minus(d, Vector.times(2, Vector.times(Vector.dot(normal, d), normal)));
        let naturalColor = Color.plus(Color.background, this.getNaturalColor(isect.thing, pos, normal, reflectDir, scene));
        let reflectedColor = (depth >= this.maxDepth) ? Color.grey : this.getReflectionColor(isect.thing, pos, normal, reflectDir, scene, depth);
        return Color.plus(naturalColor, reflectedColor);
    }
    getReflectionColor(thing, pos, normal, rd, scene, depth) {
        return Color.scale(thing.surface.reflect(pos), this.traceRay({ start: pos, dir: rd }, scene, depth + 1));
    }
    getNaturalColor(thing, pos, norm, rd, scene) {
        let addLight = (col, light) => {
            let ldis = Vector.minus(light.pos, pos);
            let livec = Vector.norm(ldis);
            let neatIsect = this.testRay({ start: pos, dir: livec }, scene);
            let isInShadow = (neatIsect === undefined) ? false : (neatIsect <= Vector.mag(ldis));
            if (isInShadow) {
                return col;
            }
            else {
                let illum = Vector.dot(livec, norm);
                let lcolor = (illum > 0) ? Color.scale(illum, light.color)
                    : Color.defaultColor;
                let specular = Vector.dot(livec, Vector.norm(rd));
                let scolor = (specular > 0) ? Color.scale(Math.pow(specular, thing.surface.roughness), light.color)
                    : Color.defaultColor;
                return Color.plus(col, Color.plus(Color.times(thing.surface.diffuse(pos), lcolor), Color.times(thing.surface.specular(pos), scolor)));
            }
        };
        return scene.lights.reduce(addLight, Color.defaultColor);
    }
    render(scene, data, x0, y0, screenWidth, screenHeight) {
        let getPoint = (x, y, camera) => {
            let recenterX = (x) => (x - (screenWidth / 2.0)) / 2.0 / screenWidth;
            let recenterY = (y) => -(y - (screenHeight / 2.0)) / 2.0 / screenHeight;
            return Vector.norm(Vector.plus(camera.forward, Vector.plus(Vector.times(recenterX(x), camera.right), Vector.times(recenterY(y), camera.up))));
        };

        for (let y = y0; y < screenHeight; y++) {
            const stride = y * screenWidth;
            for (let x = x0; x < screenWidth; x++) {
                const index = (x + stride) * 4;
                let ray = this.traceRay({ start: scene.camera.pos, dir: getPoint(x, y, scene.camera) }, scene, 0);
                let color = Color.toDrawingColor(ray);
                data[index + 0] = color.r;
                data[index + 1] = color.g;
                data[index + 2] = color.b;
                data[index + 3] = 255;
            }
        }
    }
}
function defaultScene() {
    return {
        things: [new Plane(new Vector(0.0, 1.0, 0.0), 0.0, Surfaces.checkerboard),
            new Sphere(new Vector(0.0, 1.0, -0.25), 1.0, Surfaces.shiny),
            new Sphere(new Vector(-1.0, 0.5, 1.5), 0.5, Surfaces.shiny)],
        lights: [{ pos: new Vector(-2.0, 2.5, 0.0), color: new Color(0.49, 0.07, 0.07) },
            { pos: new Vector(1.5, 2.5, 1.5), color: new Color(0.07, 0.07, 0.49) },
            { pos: new Vector(1.5, 2.5, -1.5), color: new Color(0.07, 0.49, 0.071) },
            { pos: new Vector(0.0, 3.5, 0.0), color: new Color(0.21, 0.21, 0.35) }],
        camera: new Camera(new Vector(3.0, 2.0, 4.0), new Vector(-1.0, 0.5, 0.0))
    };
}

export default function render(ctx, x, y, width, height) {
    let rayTracer = new RayTracer();
    const img = ctx.createImageData(width, height);
    rayTracer.render(defaultScene(), img.data, x, y, width, height);
    ctx.putImageData(img, -x, -y);
}
